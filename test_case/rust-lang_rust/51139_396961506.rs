bash
git clone https://github.com/vakaras/rust.git -b issue-50716 issue-50716
cd issue-50716/
./x.py build
./x.py test src/tools/cargotest src/tools/cargo
