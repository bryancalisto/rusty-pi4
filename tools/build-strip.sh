dir="$(dirname $0)"
echo $dir
$dir/build.sh
$dir/strip-unused-code.sh