# i3-bindings
WIP: Simple tool that reads the i3 config files and shows a table with the bindings defined therein

As a reference see [this](https://github.com/AndrewOlsen/i3-used-keybinds) other project. 

See [here](https://i3wm.org/docs/userguide.html#configuring) for locations of configuration file

## Potential-extensions

- ability to tag bindings with a category, possibly by using comments of the form 
  `# Category: <category` to separate the bindings.
    - Draw the table taking into account these categories
- read configuration file from dynamic location (based on what is said in the documentation)
- provide option to specify custom config location
- provide option to print bindings as _csv_ so that they can potentially be processed by another
  application.
- print information about rules:
    - number, which key is $mod`