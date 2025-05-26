use std::time::Duration;

use processing::Processor;

#[unsafe(no_mangle)]
pub extern "C" fn do_add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn test_do_add() {
    assert_eq!(do_add(1, 2), 3);
    assert_ne!(do_add(5, 2), 3);
}

#[repr(C)]
pub enum ProcessingStatus {
    Ok,
    Timeout,
    Overflow,
    EnqueFailed,
    OtherError,
}

#[unsafe(no_mangle)]
pub extern "C" fn processor_create() -> *mut Processor {
    let processor = Processor::run();
    let processor = Box::new(processor);
    Box::into_raw(processor)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn processor_enque_add(processor: *mut Processor, left: i32, right: i32) -> ProcessingStatus {
    let processor = unsafe {
        assert!(!processor.is_null());
        &*processor
    };

    match processor.enque_processing(processing::Processing::Add { left, right }) {
        Ok(_) => ProcessingStatus::Ok,
        Err(_) => ProcessingStatus::EnqueFailed,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn processor_poll_result(processor: *mut Processor, result_value: *mut i32, timeout_millis: u64) -> ProcessingStatus {
    let processor = unsafe {
        assert!(!processor.is_null());
        &*processor
    };

    let poll_result = match processor.poll_results(Duration::from_millis(timeout_millis)) {
        Ok(result) => match result {
            Ok(processing_success) => match processing_success {
                processing::ProcessingSuccess::Add(add_value) => Ok(add_value),
                processing::ProcessingSuccess::Sub(sub_value) => Ok(sub_value),
            },
            Err(processing_error) => match processing_error {
                processing::ProcessingError::Overflow => Err(ProcessingStatus::Overflow),
            },
        },
        Err(_) => Err(ProcessingStatus::OtherError),
    };

    match poll_result {
        Ok(value) => {
            let result_value = unsafe {
                assert!(!result_value.is_null());
                &mut *result_value
            };
            *result_value = value;
            ProcessingStatus::Ok
        },
        Err(e) => e,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn processor_free(processor: *mut Processor) {
    if processor.is_null() {
        return;
    }

    unsafe {
        let processor = Box::from_raw(processor);
        drop(processor);
    }
}

mod processing {
    use std::time::Duration;

    pub type ProcessingResult = Result<ProcessingSuccess, ProcessingError>;

    pub enum Processing {
        Add {
            left: i32,
            right: i32
        },
        Sub {
            left: i32,
            right: i32
        },
    }

    #[derive(Debug)]
    pub enum ProcessingSuccess {
        Add(i32),
        Sub(i32)
    }
    
    #[derive(Debug)]
    pub enum ProcessingError {
        Overflow
    }

    pub struct Processor {
        thread_handle: std::thread::JoinHandle<()>,
        processing_tx: std::sync::mpsc::Sender<Processing>,
        results_rx: std::sync::mpsc::Receiver<ProcessingResult>,
    }

    impl Processor {
        pub fn run() -> Self {
            let (processing_tx, processing_rx) = std::sync::mpsc::channel::<Processing>();

            let (results_tx, results_rx) = std::sync::mpsc::channel::<ProcessingResult>();

            let thread_handle = std::thread::spawn(move || {
                loop {
                    
                    match processing_rx.recv() {
                        Ok(processing) => match processing {
                            Processing::Add { left, right } => {
                                std::thread::sleep(Duration::from_millis(100));
                                let result = match left.checked_add(right) {
                                    Some(result) => Ok(ProcessingSuccess::Add(result)),
                                    None => Err(ProcessingError::Overflow),
                                };
                                if results_tx.send(result).is_err() {
                                    println!("Noone need processing results anymore :(");
                                }
                            },
                            Processing::Sub { left, right } => {
                                std::thread::sleep(Duration::from_millis(250));

                                let result = match left.checked_sub(right) {
                                    Some(result) => Ok(ProcessingSuccess::Sub(result)),
                                    None => Err(ProcessingError::Overflow),
                                };
                                if results_tx.send(result).is_err() {
                                    println!("Noone need processing results anymore :(");
                                }
                            },
                        },
                        Err(_) => {
                            println!("Noone will ask for work anymore, break! :)");
                            break;
                        }
                    }

                }
            });

            Self { thread_handle, processing_tx, results_rx }
        }

        pub fn enque_processing(&self, processing: Processing) -> Result<(), std::sync::mpsc::SendError<Processing>> {
            self.processing_tx.send(processing)
        }

        pub fn poll_results(&self,timeout: Duration) -> Result<ProcessingResult, std::sync::mpsc::RecvTimeoutError> {
            self.results_rx.recv_timeout(timeout)
        }
        
    }

    #[cfg(test)]
    mod tests {
        use std::time::Duration;

        use crate::processing::{ProcessingError, ProcessingSuccess};

        use super::Processor;

        #[test]
        fn test_adding_success() {
            let processor = Processor::run();
            let left = 1;
            let right = 2;
            assert!(processor.enque_processing(super::Processing::Add { left, right }).is_ok());
            let result = processor.poll_results(Duration::from_millis(500));
            match result {
                Ok(Ok(ProcessingSuccess::Add(v))) if v == left + right => {},
                _ => { panic!("Bad processing result = {result:?}") }
            };
        }

        #[test]
        fn test_adding_timeout() {
            let processor = Processor::run();
            let left = 1;
            let right = 2;
            assert!(processor.enque_processing(super::Processing::Add { left, right }).is_ok());
            let result = processor.poll_results(Duration::from_millis(20));
            assert!(result.is_err());
        }

        #[test]
        fn test_adding_overflow() {
            let processor = Processor::run();
            let left = std::i32::MAX;
            let right = 1;
            assert!(processor.enque_processing(super::Processing::Add { left, right }).is_ok());
            let result = processor.poll_results(Duration::from_millis(500));
            match result {
                Ok(Err(ProcessingError::Overflow)) => {},
                _ => { panic!("Bad processing result = {result:?}") }
            };
        }
    }
}