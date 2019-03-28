FROM marionebl/rust-cubicle

ADD . ./

RUN cargo build --tests
RUN rls-build