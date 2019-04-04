FROM brainpower/rust-cubicle

ADD . /root/project/

RUN cargo build --tests
RUN rls-build
