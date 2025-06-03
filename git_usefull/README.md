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

## Tag & GitHub release

Tag recent commit:

```sh
git tag -a <annotation like v0.1.0> -m "<msg>"
```

Tag commit by commit hash:

```sh
git tag -a <annotation like v0.1.0> <hash> -m "<msg>"
```

Delete tag locally:

```sh
git tag -d v0.1.0
```

Delete tag remotely:

```sh
git push origin --delete tag v0.1.0
```

Push all tags:

```sh
git push origin --tags
```

List tags:

```sh
git tag --list
```

Release:
1. GitHub > Releases
2. `Tags` tab > select newly created tag > `Create release from tab`
3. Set name, example `V0.2.0`
4. Attach binaries
5. Done!

## Stash

## Combine commits
