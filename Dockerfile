FROM brainpower/rust-cubicle

ADD . /repo
WORKDIR /repo

RUN cargo build --tests
RUN rls-build
