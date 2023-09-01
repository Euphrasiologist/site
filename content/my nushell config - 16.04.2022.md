# Nushell

I like Bash. It's a nice shell, and I'm sure I have not explored even most of what it can do. I've found a new shell recently though, <a href="https://www.nushell.sh/">Nushell</a>! It's a promising outlook, and I really like the nu language too. Nu takes a more data focused approach to text manipulation in the shell, which is refreshing.

Nushell has re-written many Unix standard programs, things like `ls`, `rm`, `which`, `cd`... which work as you would expect coming from another shell. The built in commands reveal themselves as they generally return a table of some sort. For example, `open`, which is a replacement for `cat`, will read data if it can into a structured table. If it can't, it will behave like `cat`. Nushell makes pipeline building easy. Say I want to look in `/usr/bin` at all the files and see if they have built-in equivalents:

```
# list all files in /usr/bin
ls /usr/bin/ 
    # get the name column
    | get name 
    # split this column on a forward slash
    | split column / 
    # get fourth column (equal to the exec name)
    | get column4 
    # on each exec name, run the which command
    | each {|e| which $e | get 0} 
    # filter the resulting table where `built-in` column is true
    | where built-in == true

# you can of course put this on a single line
ls /usr/bin/ | get name | split column / | get column4 | each {|e| which $e | get 0} | where built-in == true
```

On my system, these are they:

`alias`, `cal`, `cd`, `clear`, `du`, `env`, `find`, `fmt`, `from`, `hash`, `last`, `open`, `seq`, `size`, `sort`, `split`, `touch`, `unalias`, `uniq`, `which`, `zip`.

Much like in Bash, where there is a `.bashrc` or a `.bash_profile` file, Nushell has an equivalent - the `config.nu` and the `env.nu` files. Using these files, I'll run through some of the functions defined in them, and give a very quick tour of the language.

## Modify the prompt indicator

I instantly wanted to modify the default colours. This seemed like an easy thing to do, and it is. If we take a look at the `env.nu` file, we see this bit:

```
# The prompt indicators are environmental variables that represent
# the state of the prompt
let-env PROMPT_INDICATOR = { "〉" }
let-env PROMPT_INDICATOR_VI_INSERT = { ": " }
let-env PROMPT_INDICATOR_VI_NORMAL = { "〉" }
let-env PROMPT_MULTILINE_INDICATOR = { "::: " }
```

Stuff to do with the prompt. The thing you'll look at every time you open Nushell is the prompt, and by default, we get a nice fat-arrow (`PROMPT_INDICATOR`) as what would usually be a `$` in Bash. But I want a fancy colour! I came up with this:

```
let-env PROMPT_INDICATOR = (echo (ansi {fg: "FFA12C", bg: ""}) " > " | str collect)
```

Which looks a bit mysterious. I'll attempt to de-mystify.

- `let-env` creates a new environmental variable
- The outer parentheses around the entire right hand statement evaluates the program and returns to the assignment
- `echo` behaves similarly as the classic `echo` program does.
- `ansi` is a function which outputs ANSI codes which Nushell interprets to change the colour of the text
- So here `echo` operates on the `ansi` and the `" > "` string...
- We can pipe the output of the `echo` into `str collect`, which unsurprisingly collects a list of items into a concatenated string

The result is a lovely new orange prompt indicator! And we've explored some of the nu language syntax.

## The left prompt

By default, the left prompt is this:

```
# def defines a function
# like python
# unlike python, whitespace != control flow...

def create_left_prompt [] {
    let path_segment = ($env.PWD)

    $path_segment
}
```

Which exposes nicely how we can define functions in nu. The square brackets can contain optional input parameters. We can define normal variables with `let`. And we can use those variables later by putting a `$` before the variable name. The `$env` is a special globally defined variable of structured data which contains lots of things. The current working directory is kept in the `$env.PWD` sub-variable. So no need for a `pwd` command.

## Going up!

Lastly I thought I'd show you something I submitted to the <a href="https://github.com/nushell/nu_scripts">Nu scripts</a> repository:

```
def up_inner [limit: int] {
  (for $e in 0..$limit { "." } | str collect)
}

# Go up a number of directories
def-env up [
    limit: int # The number of directories to go up
  ] {
    cd (up_inner $limit)
}
```

Nu has for-loops, which is nice. `up_inner` simply loops until the user input limit, and concatenates as many dots as the limit will allow. In Nushell `cd ...` will go up 2 directories. The `up` function has to be an environmental function (`def-env`), otherwise we only change directory within the body of the function itself (so the user experiences no such changes in directory). Strategically placed commenting gets rendered by nu as a nice help screen:

<div style="text-align:center;">
    <img src="./../img/my%20nushell%20config%20-%2011.04.2022/Screenshot%202022-04-16%20at%2010.16.06.png" width="65%">
</div>

Well that was a bit longer than intended, but I hope it encourages the reader to try out Nushell!