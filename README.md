# (G)it (Ig)nore

This is a simple tool to generate a `.gitignore` file for your project. It is based on the same common templates used by [gitignore.io](https://www.gitignore.io/) but it is a standalone tool that you can use offline.

## Installation

(coming soon)

## Usage

### Inline mode

You can use the tool in inline mode by passing the names of the templates you want to use as arguments:

```bash
$ gig python vscode
```

By default, this will print the content of the created `.gitignore` file to the standard output. You can redirect it to a file if you want to save it.

### Interactive mode

You can also use the tool in interactive mode by running it without arguments:

```bash
$ gig
```

This will open a prompt where you can select the templates you want to use. You can select multiple templates by separating them with spaces and has hints to help you choose the right templates. When you are done, press `Enter` to generate the `.gitignore` file to your current directory.

![Inline mode](res/example.gif)