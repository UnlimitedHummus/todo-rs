# TODO-rs 
Simple command line todo utility. This cli tool is useful for quickly adding todos to a folder in your project.
Sometimes, you want to keep track of tasks that are specific to a folder you're working on.
This is the ideal tool if your're frequently leaving todo files in your personal projects, where anything more would be overkill.

# Usage

To create an empty `.todo` file in your directory, simply run:

```bash
todo create
```

Once you have a `.todo` file, you can add entries with:

```bash
> todo add "My first todo"
Added: "My first todo"
```

This will show up as `[ ] My first todo` in the `.todo` file so that you
can manually edit todos quickly and easily understand changes in a commit diff.
Now lets add a second item:

```bash
> todo add "My second todo"
Added: "My second todo"
> todo list
1 [ ] My first todo
2 [ ] My second todo

``` 

I feel like the usafe becomes pretty self explanatory from here. 

```bash
> todo check 1
[x] My first todo
> todo list
1 [ ] My second todo

2 [x] My first todo
> todo remove 1
Removed: [ ] My second todo 
> todo list

2 [ ] My first todo
```

To remove the `.todo` file, simply run `todo destroy` or simply delete the .todo file.

```bash
> todo destroy
Deleted: .todo
```

Simply running `todo` will display a help menu.
```
Magnus Balzer <magnus.balzer@gmail.com>
Simple todo lists

USAGE:
    todo <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add        Add an item to the todo list
    check      Mark an item on the list as done
    create     Create a new todo list
    destroy    Destroy the todo list
    help       Print this message or the help of the given subcommand(s)
    list       List all items on the list
    remove     Remove an item from the list
```

# Installing

To install todo, simply run this from the root of the project.
```bash
cargo install --path .
```

