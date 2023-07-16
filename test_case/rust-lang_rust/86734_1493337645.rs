
sudo pacman -Fy
for el in $(find /usr -type f) ; do
    if [ -z "$(pacman -F "$el")" ] ; then
        echo "$el was not installed by the package manager."
        sudo rm "$el"
    fi
done
