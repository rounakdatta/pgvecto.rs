ARG TAG
ARG POSTGRES_VERSION
FROM scratch as nothing
ARG TARGETARCH
FROM rounakdatta/pgvecto-rs-binary:${TAG}-${TARGETARCH} as binary

FROM postgres:14.5
COPY --from=binary /pgvecto-rs-binary-release.deb /tmp/vectors.deb
RUN apt-get install -y /tmp/vectors.deb && rm -f /tmp/vectors.deb
CMD ["postgres","-c","shared_preload_libraries=vectors.so"]
