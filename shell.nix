let
  self = import ./.;
in self.devShells.default or { } // {
  __functor = _: { pkgs ? null, ... }@args: (self args).devShells.default;
}
