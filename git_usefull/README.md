# Git Usefull

## Set username

Setting global username/email:
```sh
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

Check:
```sh
git config --list
```

## Git init locally, add Github origin

```sh
git remote add origin https://github.com/<github_user>/<repo_name>.git
```

Check:
```sh
git remote -v
```

First push to master/main:
```sh
git push -u origin master
```

Then you can use just `git push`, `git pull`.

## Tag & release

## Stash

## Combine commits