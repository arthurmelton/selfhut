# [git.arthurmelton.com](git.arthurmelton.com)

This is my git site, it took insperation from [Sourcehut](https://sourcehut.org/).
Its not a fork but its practecly a remake, just for my use case. Please support
Drew DeVault at [https://sourcehut.org/pricing/](https://sourcehut.org/pricing/).
If you are going to run this yourself. I really recommend supporting Drew DeVault
because without him this would not exist!

## Email

This can also run on any email server though a open imap connection. The actual
mailing list and creation of accounts are handled through different programs.
For every repo you need to make a email for it (ex. if I have a repo called
"My-Cool-Program" you need to make an email for it called 
"My-Cool-Program@example.com").

## Hosting

Simpily run the binary and a website is going to be hosted at port 8000. I would
also recoment using something like nginx to change the port and to do caching. If 
you want to change the config (this would make sense because at default it only
has a example user), edit the file in `$(XDG_CONFIG_HOME)/git-server/git-server.toml`
to your liking.

## Making a new git

To make a new git repo you need to run 
```sh
git init --bare my-repo.git
```
After you do this you will need to cd into the directory and make a file at the 
path `hooks/post-update` and set its contents to
```sh
#!/bin/sh

exec git update-server-info
```
This should create the git and make it so that people can actually clone it!
