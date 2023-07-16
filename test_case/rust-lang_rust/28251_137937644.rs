 bash
$ cat test.md
Test  | Table
------|-------------
t = b | `x \| y \| z`

$ ./hoedown --tables test.md
<table>
<thead>
<tr>
<th>Test</th>
<th>Table</th>
</tr>
</thead>

<tbody>
<tr>
<td>t = b</td>
<td>`x \</td>
</tr>
</tbody>
</table>
