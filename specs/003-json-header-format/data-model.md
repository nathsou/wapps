# Data Model: JSON Header Format

## Enitites

### WappFile (Binary Container)
Represents the physical file layout on disk.

| Field | Type | Size | Description |
|-------|------|------|-------------|
| Magic | Binary | 4 bytes | Literal `b"WAPP"` (0x57 0x41 0x50 0x50) |
| Version | Integer | 4 bytes | Unsigned 32-bit Little Endian. Must be `1`. |
| HeaderLength | Integer | 4 bytes | Unsigned 32-bit Little Endian. Size of JSON data in bytes (`N`). |
| HeaderData | String | `N` bytes | UTF-8 encoded JSON string. |
| Payload | Binary | Variable | The WebAssembly module binary. |

### WappMetadata (JSON)
The structured data contained within `HeaderData`.

```json
{"name": "Application Name"}
```

#### Fields

- **name** (Optional, String): The display name of the application. Used for window titles.
  - *Validation*: If present, must be a string. Empty strings are allowed but fall back to filename.
- **description** (Optional, String): A short description of the application.
  - *Validation*: String. (Note: Not strictly required by User Story for window title, but good practice to keep).
- *any other keys*: Allowed and ignored by the host runtime, but preserved for tools.

## State Transitions
N/A - Start-up usage only.
