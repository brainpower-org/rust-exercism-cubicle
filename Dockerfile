FROM brainpower/rust-cubicle

USER rust

ADD . /root/project/
WORKDIR /root/project

RUN cargo build --tests
RUN rls-build