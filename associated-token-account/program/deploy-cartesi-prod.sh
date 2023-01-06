
cd ../../../rollups-examples
export ROLLUPS_EXAMPLES=`pwd`
cd -

cd ../../../cartesi-solana
export CARTESI_SOLANA=`pwd`
cd -

cd ../../token
export TOKEN_PROGRAM=`pwd`
cd -

cd ../../memo
export MEMO=`pwd`
cd -

docker run \
    -v `pwd`:/workdir \
    -v $ROLLUPS_EXAMPLES:/rollups-examples \
    -v $CARTESI_SOLANA:/cartesi-solana \
    -v $TOKEN_PROGRAM:/token \
    -v $MEMO:/memo \
    -w /workdir \
    --rm \
    cartesi/toolchain:0.11.0 \
    ./build-prod.sh
