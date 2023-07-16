
[features]
default = ["metal"]
dx12 = ["gfx-backend-dx12"]
empty = ["gfx-backend-empty"]
gl = ["gfx-backend-gl"]
metal = ["gfx-backend-metal"]
vulkan = ["gfx-backend-vulkan"]

[dependencies]
gfx-backend-dx12 = { version = "0.6.3", optional = true }
gfx-backend-empty = { version = "0.6.0", optional = true }
gfx-backend-gl = { version = "0.5.1", optional = true }
gfx-backend-metal = { version = "0.6.2", optional = true }
gfx-backend-vulkan = { version = "0.6.1", optional = true }
gfx-hal = "0.6.0"
