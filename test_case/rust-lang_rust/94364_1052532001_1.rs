
RUN mkdir /libstdc++ && cp /usr/local/x86_64-linux-musl/lib/libstdc++.a /libstdc++/
ENV RUSTFLAGS="-L /libstdc++"
