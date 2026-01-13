{
  rustPlatform,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  name = "sl";
  src = ./.;

  nativeBuildInputs = [pkg-config];
  cargoLock.lockFile = ./Cargo.lock;
}
