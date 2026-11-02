pipewire &
sleep 1
pipewire-pulse &
wireplumber &
picom -b &
dunst &
flameshot &

setxkbmap -layout us,ru -option grp:alt_shift_toggle &
xset r rate 250 30 &

feh --bg-fill /home/caylann/dotfiles/assets/av1.jpg &

exec dbus-run-session dwm
