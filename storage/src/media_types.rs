#[derive(Clone, Debug, Copy, PartialEq)]
pub enum MediaType {
  None,            // Not support
  ManifestV1,      // schema1 (existing manifest format)
  ManifestV2,      // New image manifest format (schemaVersion = 2)
  ManifestList,    // Manifest list, aka "fat manifest"
  ContainerConfig, // Container config JSON
  Layer,           // "Layer", as a gzipped tar
  ForeignLayer,    // "Layer", as a gzipped tar that should never be pushed
  PluginConfig,    // Plugin config JSON
  // OCIs
  ContentDescriptor, // OCI Content Descriptor
  Layout,            // OCI Layout
  ImageIndex,        // OCI Image Index
  ImageManifest,     // Image manifest
  ImageConfig,       // Image config
  ImageTarLayer,     // "Layer", as a tar archive
  ImageGzipLayer,    // "Layer", as a tar archive compressed with gzip
  ImageZstdLayer,    // "Layer", as a tar archive compressed with zstd
  ImageNdTarLayer,   // "Layer", as a tar archive with distribution restrictions
  ImageNdGzipLayer, // "Layer", as a tar archive with distribution restrictions compressed with gzip
  ImageNdZstdLayer, // "Layer", as a tar archive with distribution restrictions compressed with zstd
}

impl MediaType {
  pub fn to_str(&self) -> &str {
    match self {
      MediaType::ManifestV1 => "application/vnd.docker.distribution.manifest.v1+json",
      MediaType::ManifestV2 => "application/vnd.docker.distribution.manifest.v2+json",
      MediaType::ManifestList => "application/vnd.docker.distribution.manifest.list.v2+json",
      MediaType::ContainerConfig => "application/vnd.docker.container.image.v1+json",
      MediaType::Layer => "application/vnd.docker.image.rootfs.diff.tar.gzip",
      MediaType::ForeignLayer => "application/vnd.docker.image.rootfs.foreign.diff.tar.gzip",
      MediaType::PluginConfig => "application/vnd.docker.plugin.v1+json",

      // OCIs
      MediaType::ContentDescriptor => "application/vnd.oci.descriptor.v1+json",
      MediaType::Layout => "application/vnd.oci.layout.header.v1+json",
      MediaType::ImageIndex => "application/vnd.oci.image.index.v1+json",
      MediaType::ImageManifest => "application/vnd.oci.image.manifest.v1+json",
      MediaType::ImageConfig => "application/vnd.oci.image.config.v1+json",
      MediaType::ImageTarLayer => "application/vnd.oci.image.layer.v1.tar",
      MediaType::ImageGzipLayer => "application/vnd.oci.image.layer.v1.tar+gzip",
      MediaType::ImageZstdLayer => "application/vnd.oci.image.layer.v1.tar+zstd",
      MediaType::ImageNdTarLayer => "application/vnd.oci.image.layer.nondistributable.v1.tar",
      MediaType::ImageNdGzipLayer => "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip",
      MediaType::ImageNdZstdLayer => "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd",
      _ => "",
    }
  }

  pub fn from_str(s: &str) -> MediaType {
    match s {
      "application/vnd.docker.distribution.manifest.v1+json" => MediaType::ManifestV1,
      "application/vnd.docker.distribution.manifest.v2+json" => MediaType::ManifestV2,
      "application/vnd.docker.distribution.manifest.list.v2+json" => MediaType::ManifestList,
      "application/vnd.docker.container.image.v1+json" => MediaType::ContainerConfig,
      "application/vnd.docker.image.rootfs.diff.tar.gzip" => MediaType::Layer,
      "application/vnd.docker.image.rootfs.foreign.diff.tar.gzip" => MediaType::ForeignLayer,
      "application/vnd.docker.plugin.v1+json" => MediaType::PluginConfig,

      // OCIs
      "application/vnd.oci.descriptor.v1+json" => MediaType::ContentDescriptor,
      "application/vnd.oci.layout.header.v1+json" => MediaType::Layout,
      "application/vnd.oci.image.index.v1+json" => MediaType::ImageIndex,
      "application/vnd.oci.image.manifest.v1+json" => MediaType::ImageManifest,
      "application/vnd.oci.image.config.v1+json" => MediaType::ImageConfig,
      "application/vnd.oci.image.layer.v1.tar" => MediaType::ImageTarLayer,
      "application/vnd.oci.image.layer.v1.tar+gzip" => MediaType::ImageGzipLayer,
      "application/vnd.oci.image.layer.v1.tar+zstd" => MediaType::ImageZstdLayer,
      "application/vnd.oci.image.layer.nondistributable.v1.tar" => MediaType::ImageNdTarLayer,
      "application/vnd.oci.image.layer.nondistributable.v1.tar+gzip" => MediaType::ImageNdGzipLayer,
      "application/vnd.oci.image.layer.nondistributable.v1.tar+zstd" => MediaType::ImageNdZstdLayer,

      _ => MediaType::None,
    }
  }

  pub fn to_string(&self) -> String {
    String::from(self.to_str())
  }
}

pub fn mt_to_str(mt: MediaType) -> String {
  mt.to_string()
}
