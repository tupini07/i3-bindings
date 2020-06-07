# i3-bindings

![CI_build_and_release](https://github.com/tupini07/i3-bindings/workflows/CI_build_and_release/badge.svg?branch=master)

Simple tool that reads the i3 config files and shows a table with the bindings defined therein. See the _Example_ below
for an example of how the printed table looks like.

If you would like to add some new functionality, or help me make my code mode idiomatic, then issues and PRs are more than welcome :smile:

## Functionality

- print the keybindings as a table
- optionally print them as a `csv` to stdout
- sort bindings by different criteria
- optionally wait for input after printing (useful if you want to do something like `alacritty -e i3-bindings -b` to open
  a temporary terminal that shows your bindings)

## Usage

See `i3-bindings --help` for a help message about the available options. However, in most cases you'll be fine with just
running `i3-bindings` and leave everything with default values.

## How to use format config file

If you want, you can specify _categories_ in your `config` file so that when printing the table bindings are logically grouped.
However this is not required.

**Categories:** for categories to be printed as in the example below, you need to add comments in
your i3 config that specify the category for a group of bindings. All bindings below the comment
will have the specified category until a new one is specified. The default category is `default`.

For example:

```
# Category: Layout

#Split in horizontal orientation
bindsym $mod+Mod1+h split h

#Split in vertical orientation
bindsym $mod+Mod1+v split v

# Category: Some other category
...etc...
```

Note that all comments in the file are completely ignored except for those about a `Category: ` (which are
used to define categories) as well as the comments on the same line as a binding (which are shown as part
of the command in the table).

## Example

When executed, the tool will read your i3 config file and print something like this to the console:

```
╔══════════╦══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗
║Category  ║Actual Binding                                                                                                    ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Layout    ║ Symbol  $mod+Mod1+h  split h                                                                                     ║
║          ║ Symbol  $mod+Mod1+v  split v                                                                                     ║
║          ║ Symbol  $mod+e       layout toggle split                                                                         ║
║          ║ Symbol  $mod+f       fullscreen toggle                                                                           ║
║          ║ Symbol  $mod+m       focus child                                                                                 ║
║          ║ Symbol  $mod+s       layout stacking                                                                             ║
║          ║ Symbol  $mod+space   floating toggle                                                                             ║
║          ║ Symbol  $mod+u       focus parent                                                                                ║
║          ║ Symbol  $mod+w       layout tabbed                                                                               ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Main      ║ Symbol  $mod+Return   alacritty                                                                                  ║
║          ║ Symbol  $mod+d        --no-startup-id rofi -show combi                                                           ║
║          ║ Symbol  $mod+n        --no-startup-id systemsettings5                                                            ║
║          ║ Symbol  Ctrl+Shift+q  kill                                                                                       ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Move Focus║ Symbol  $mod+Down         focus down                                                                             ║
║          ║ Symbol  $mod+Left         focus left                                                                             ║
║          ║ Symbol  $mod+Right        focus right                                                                            ║
║          ║ Symbol  $mod+Shift+Down   move down                                                                              ║
║          ║ Symbol  $mod+Shift+Left   move left                                                                              ║
║          ║ Symbol  $mod+Shift+Right  move right                                                                             ║
║          ║ Symbol  $mod+Shift+Up     move up                                                                                ║
║          ║ Symbol  $mod+Shift+h      move left                                                                              ║
║          ║ Symbol  $mod+Shift+j      move down                                                                              ║
║          ║ Symbol  $mod+Shift+k      move up                                                                                ║
║          ║ Symbol  $mod+Shift+l      move right                                                                             ║
║          ║ Symbol  $mod+Up           focus up                                                                               ║
║          ║ Symbol  $mod+h            focus left                                                                             ║
║          ║ Symbol  $mod+j            focus down                                                                             ║
║          ║ Symbol  $mod+k            focus up                                                                               ║
║          ║ Symbol  $mod+l            focus right                                                                            ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Personal  ║ Symbol  $mod+$alt+b            alacritty -e i3-bindings                                                          ║
║          ║ Symbol  $mod+$alt+c            $EDITOR ~/.config/i3/config                                                       ║
║          ║ Symbol  $mod+$alt+d            dolphin                                                                           ║
║          ║ Symbol  $mod+$alt+e            $EDITOR                                                                           ║
║          ║ Symbol  $mod+$alt+g            chromium                                                                          ║
║          ║ Symbol  $mod+$alt+q            fish -c ask-and-run-command-in-new-term                                           ║
║          ║ Symbol  $mod+$alt+r            alacritty -e ranger                                                               ║
║          ║ Symbol  $mod+$alt+s            spotify-wrapper                                                                   ║
║          ║ Code    $mod+34                picom            #34 - [                                                          ║
║          ║ Code    $mod+35                killall picom    #35 - ]                                                          ║
║          ║ Code    $mod+Shift+35          ~/.config/polybar/launch.sh                                                       ║
║          ║ Symbol  $mod+equal             gaps inner all plus 5                                                             ║
║          ║ Symbol  $mod+minus             gaps inner all minus 5                                                            ║
║          ║ Symbol  Ctrl+$alt+l            ~/.shellscripts/lock-screen.fish                                                  ║
║          ║ Symbol  Ctrl+Print             scrot '%Y%m%d_%H%M%S.png' -e 'mv $f ~/Pictures/Screenshots/'                      ║
║          ║ Symbol  XF86AudioLowerVolume   pactl set-sink-volume 0 -2%                                                       ║
║          ║ Symbol  XF86AudioMicMute       pactl set-source-mute 1 toggle                                                    ║
║          ║ Symbol  XF86AudioMute          pactl set-sink-mute 0 toggle                                                      ║
║          ║ Symbol  XF86AudioRaiseVolume   pactl set-sink-volume 0 +2%                                                       ║
║          ║ Symbol  XF86MonBrightnessDown  fish -c "increase-display-backlight-by -30"                                       ║
║          ║ Symbol  XF86MonBrightnessUp    fish -c "increase-display-backlight-by 30"                                        ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Resize    ║ Symbol  $mod+r  mode "resize"                                                                                    ║
║          ║ Symbol  $mod+r  mode "default"                                                                                   ║
║          ║ Symbol  Down    resize grow height 8 px or 2 ppt                                                                 ║
║          ║ Symbol  Escape  mode "default"                                                                                   ║
║          ║ Symbol  Left    resize shrink width 8 px or 2 ppt                                                                ║
║          ║ Symbol  Right   resize grow width 8 px or 2 ppt                                                                  ║
║          ║ Symbol  Up      resize shrink height 8 px or 2 ppt                                                               ║
║          ║ Symbol  h       resize shrink width 8 px or 2 ppt                                                                ║
║          ║ Symbol  j       resize grow height 8 px or 2 ppt                                                                 ║
║          ║ Symbol  k       resize shrink height 8 px or 2 ppt                                                               ║
║          ║ Symbol  l       resize grow width 8 px or 2 ppt                                                                  ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Restart   ║ Symbol  $mod+Shift+e  "i3-nagbar -t warning -m 'Do you really want to exit i3?' -b 'Yes, exit i3' 'i3-msg exit'" ║
║          ║ Symbol  $mod+Shift+r  restart                                                                                    ║
╟──────────╫──────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║Workspaces║ Symbol  $mod+0           workspace 10                                                                            ║
║          ║ Symbol  $mod+1           workspace 1                                                                             ║
║          ║ Symbol  $mod+2           workspace 2                                                                             ║
║          ║ Symbol  $mod+3           workspace 3                                                                             ║
║          ║ Symbol  $mod+4           workspace 4                                                                             ║
║          ║ Symbol  $mod+5           workspace 5                                                                             ║
║          ║ Symbol  $mod+6           workspace 6                                                                             ║
║          ║ Symbol  $mod+7           workspace 7                                                                             ║
║          ║ Symbol  $mod+8           workspace 8                                                                             ║
║          ║ Symbol  $mod+9           workspace 9                                                                             ║
║          ║ Symbol  $mod+Shift+0     move container to workspace 10                                                          ║
║          ║ Symbol  $mod+Shift+1     move container to workspace 1                                                           ║
║          ║ Symbol  $mod+Shift+2     move container to workspace 2                                                           ║
║          ║ Symbol  $mod+Shift+3     move container to workspace 3                                                           ║
║          ║ Symbol  $mod+Shift+4     move container to workspace 4                                                           ║
║          ║ Symbol  $mod+Shift+5     move container to workspace 5                                                           ║
║          ║ Symbol  $mod+Shift+6     move container to workspace 6                                                           ║
║          ║ Symbol  $mod+Shift+7     move container to workspace 7                                                           ║
║          ║ Symbol  $mod+Shift+8     move container to workspace 8                                                           ║
║          ║ Symbol  $mod+Shift+9     move container to workspace 9                                                           ║
║          ║ Symbol  $mod+Shift+m     move workspace to output left                                                           ║
║          ║ Symbol  $mod+Shift+n     move workspace to output left                                                           ║
║          ║ Symbol  Ctrl+$mod+Left   workspace prev                                                                          ║
║          ║ Symbol  Ctrl+$mod+Right  workspace next                                                                          ║
╚══════════╩══════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝
```
