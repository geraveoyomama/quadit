FROM docker.io/fedora@sha256:d0207dbb078ee261852590b9a8f1ab1f8320547be79a2f39af9f3d23db33735e as build

RUN dnf install -y gcc openssl-devel && \
    rm -rf /var/cache/dnf && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR "/app-build"

ENV PATH=/root/.cargo/bin:${PATH}

COPY ./src ./src
COPY Cargo.toml  ./
RUN cargo build --release

FROM docker.io/fedora@sha256:d0207dbb078ee261852590b9a8f1ab1f8320547be79a2f39af9f3d23db33735e

ENV container docker
RUN dnf -y update; dnf clean all
RUN dnf -y install systemd openssl-devel; dnf clean all; \
    (cd /lib/systemd/system/sysinit.target.wants/; for i in *; do [ $i == systemd-tmpfiles-setup.service ] || rm -f $i; done); \
    rm -f /lib/systemd/system/multi-user.target.wants/*; \
    rm -f /etc/systemd/system/*.wants/*; \
    rm -f /lib/systemd/system/local-fs.target.wants/*; \
    rm -f /lib/systemd/system/sockets.target.wants/*udev*; \
    rm -f /lib/systemd/system/sockets.target.wants/*initctl*; \
    rm -f /lib/systemd/system/basic.target.wants/*; \
    rm -f /lib/systemd/system/anaconda.target.wants/*;
VOLUME [ "/sys/fs/cgroup" ]

WORKDIR "/app"
COPY --from=build /app-build/target/release/quadit ./

CMD [ "./quadit" ]
