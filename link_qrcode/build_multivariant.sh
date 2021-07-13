CURRENT="$PWD"
OUT_FOLDER="multivariant"
ORIGINAL_BITCODE=$1
MEWE_FOLDER=$2
VARIANTS_FOLDER=$3
shift 3

args="--variants-no-inline --instrument-only-variants --instrument-include=run_qr,run_qr_str"



CURRENT=1
MULTIVARIANTS="$(find ./bitcodes -name '*.bc' -exec echo -n ","{} \;)" #"$(find $VARIANTS_FOLDER -name '*.bc' -exec echo -n ","{} \;)"

echo $MULTIVARIANTS
# Merge all in one big fat

# Link all originals


$MEWE_FOLDER/build/crow-linker $ORIGINAL_BITCODE "$OUT_FOLDER/allinone.multivariant.i.bc" -complete-replace=false -merge-function-switch-cases --replace-all-calls-by-the-discriminator --instrument-function --override -crow-merge-debug-level=2 -crow-merge-skip-on-error $args -crow-merge-bitcodes="$MULTIVARIANTS"  2> multivariant/i.map.txt


$MEWE_FOLDER/build/crow-linker $ORIGINAL_BITCODE "$OUT_FOLDER/allinone.multivariant.bc" -complete-replace=false --replace-all-calls-by-the-discriminator -merge-function-switch-cases  --override -crow-merge-debug-level=2 -crow-merge-skip-on-error -crow-merge-bitcodes="$MULTIVARIANTS"


llvm-dis "$OUT_FOLDER/allinone.multivariant.i.bc" -o multivariant/allinone.multivariant.i.bc.ll

#find out_group -name "*.multivariant.bc" -exec cp -f {} $OUT_FOLDER/ \;