 bash
git clone https://github.com/pingcap/tidb.git $GOPATH/src/github.com/pingcap/tidb
cd $GOPATH/src/github.com/pingcap/tidb
make 
# the tidb-server is installed in $GOPATH/src/github.com/pingcap/tidb/bin directory 

git clone https://github.com/pingcap/pd.git $GOPATH/src/github.com/pingcap/pd
cd $GOPATH/src/github.com/pingcap/pd
make
# the pd-server is installed in $GOPATH/src/github.com/pingcap/pd/bin directory 
