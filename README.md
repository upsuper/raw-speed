# raw-speed

`raw-speed` is a tool for measuring the network speed between two devices.

It need one device to be the sever via running
```
raw-speed server
```
and the other device to be the client via running
```
raw-speed client <mode> <server>
```

`<mode>` can be one of `up`, `down`, and `both`, meaning measuring upstream, downstream, or both together.

`<server>` is the address of the server.

## License

Copyright (C) 2018,2020 Xidorn Quan

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.
