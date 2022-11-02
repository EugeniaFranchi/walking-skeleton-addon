FROM  continuumio/anaconda3
WORKDIR /app
ADD requirements.txt requirements.txt

RUN apt-get update
RUN apt-get -y install make build-essential curl 

# Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | bash -s -- -y

# Node
RUN curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && apt-get install -y nodejs

# Tensorflow model
RUN apt-get install ffmpeg libsm6 libxext6  -y
RUN pip install -r requirements.txt

# Jupyter notebook port
EXPOSE 8888
