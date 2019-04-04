FROM brainpower/rust-cubicle

ADD . /root/project/

RUN rustup default stable
RUN cargo build --tests
RUN rls-build
