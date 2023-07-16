bash
$ for file in $(find target-nocov -name '*.r[lm]*'); do \
  covfile="${file/target-nocov/target-cov}"; \
  nocov=$(stat -c%s $file); cov=$(stat -c%s $covfile); \
  printf "%+7.2f%% = %5s / %5s: %s\n" \
      "$( echo "scale=4; 100 * ($cov / $nocov) - 100" | bc )" \
      "$(numfmt --to=si $nocov)" \
      "$(numfmt --to=si $cov)" \
      "${file/-nocov/}"; \
done
