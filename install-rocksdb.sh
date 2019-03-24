yum install -y snappy snappy-devel &&
yum install -y zlib zlib-devel &&
yum install -y bzip2 bzip2-devel &&
yum install -y lz4-devel &&
yum install -y libasan
wget https://github.com/facebook/zstd/archive/v1.1.3.tar.gz
mv v1.1.3.tar.gz zstd-1.1.3.tar.gz
tar zxvf zstd-1.1.3.tar.gz
cd zstd-1.1.3
make && make install
