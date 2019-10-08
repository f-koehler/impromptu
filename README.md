# [WIP] impromptu

[![Build Status](https://travis-ci.org/f-koehler/impromptu.svg?branch=master)](https://travis-ci.org/f-koehler/impromptu)

Generate your prompt with an efficient compiled rust program and make it look identical across shells.


## Using in your shell

### zsh

Add this line to your `~/.zshrc`:
```zsh
precmd() {
    PROMPT=$'$(/path/to/impromptu "$?" "$(jobs -l)")'
}
```


### bash

Add this line to your `~/.bashrc`:
```bash
PS1='$(/path/to/impromptu "$?" "$(jobs -l)")'
```
