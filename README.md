# Parkour legacy helper
## Keybinds
* R for double space (for wallboosting)
* Left Alt for quick space (for dashing)
## Compatability
This program uses rdev to do its work so it should work on most platforms (has only been tested on nixos though) (also byfron could be a problem on some platforms)
## Contribution
If you would like to improve this go for it, I will be lookign out for any contributions. Also if you want to suggest a feature or report a bug feel free to on issues
## Instalation
To instal:
For nixos/nix : 
``` bash
git clone https://github.com/quotequack/parkourlegacyhelper
cd parkourlegacyhelper
nix profile install .#parkour
```
For linux :
``` bash
git clone https://github.com/quotequack/parkourlegacyhelper
cd parkourlegacyhelper
cargo build --release
sudo cp ./target/debug/parkour /usr/bin/
```
For windows and mac :
idk figure it out and edit this to help others
## Usage
After instalation simply go to your terminal of choice and run:
``` bash
parkour
```
by the way on some devices you may need to use sudo:
``` bash
sudo parkour
```
Now open roblox/sober and join the game. You should have access to all the keybinds!
