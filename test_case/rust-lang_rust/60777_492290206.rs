yaml
      - restore_cache:
          keys:
            - ruby-dependency-cache--v4--{{ arch }}-{{ .Branch }}-{{ checksum "Gemfile.lock" }}
            - ruby-dependency-cache--v4--{{ arch }}-{{ .Branch }}-
            - ruby-dependency-cache--v4--{{ arch }}-
