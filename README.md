<h1 align="center">Bank smart contract</h1>
<img src="https://media3.giphy.com/media/iG4zIb2xaxCBbZ3uei/giphy.gif?cid=ecf05e47jepsoc6iqg175d7y1caf5m3ernz0c6p3kox8oby4&rid=giphy.gif&ct=g" align="right" width="400">
<p>
  To use this code, you must enter these commands in your terminal:
</p>
<p>
  <strong>First:</strong> Install <em>Rust</em> with this command: <code>curl https://sh.rustup.rs -sSf | sh </code>. <br>
  
  <strong>Second:</strong> Install the toolchain nightly: <br>
  <code>rustup default stable</code> <br>
  <code>rustup update</code> <br>
  <code>rustup update nightly</code> <br>
  <code>rustup component add rust-src</code> <br>
  <code>rustup component add rust-src --toolchain nightly</code> <br>
  <code>rustup target add wasm32-unknown-unknown --toolchain nightly</code>. <br>
  
  <strong>Third:</strong> Install other necessary dependencies: <br>
  <code>cargo install cargo-dylint dylint-link</code>. <br>
  
  <strong>Fourth:</strong> After you've installed the package: <br>
  <code>cargo install cargo-contract --force --locked</code>.
  
  <strong>Finally:</strong> In your terminal: <br>
  <code>cargo contract build</code>. <br>
  
  This command will compile the smart contract and generate the metadata for our file. <br>
  
  <em>Congratulations, you have just created your first smart contract using Ink!</em>
</p>
