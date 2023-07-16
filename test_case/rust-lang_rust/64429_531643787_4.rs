\n"
2019-09-16T05:15:42.9258036Z   },
2019-09-16T05:15:42.9258079Z   "level": "error",
2019-09-16T05:15:42.9258132Z   "spans": [
2019-09-16T05:15:42.9258262Z       "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9258310Z       "byte_start": 471,
2019-09-16T05:15:42.9258370Z       "byte_end": 475,
2019-09-16T05:15:42.9258412Z       "line_start": 12,
2019-09-16T05:15:42.9258412Z       "line_start": 12,
2019-09-16T05:15:42.9258527Z       "line_end": 12,
2019-09-16T05:15:42.9258595Z       "column_start": 12,
2019-09-16T05:15:42.9258639Z       "column_end": 16,
2019-09-16T05:15:42.9258681Z       "is_primary": true,
2019-09-16T05:15:42.9258737Z       "text": [
2019-09-16T05:15:42.9258779Z         {
2019-09-16T05:15:42.9258823Z           "text": "    let x: Iter;",
2019-09-16T05:15:42.9258869Z           "highlight_start": 12,
2019-09-16T05:15:42.9258929Z           "highlight_end": 16
2019-09-16T05:15:42.9259011Z       ],
2019-09-16T05:15:42.9259011Z       ],
2019-09-16T05:15:42.9259070Z       "label": "not found in this scope",
2019-09-16T05:15:42.9259116Z       "suggested_replacement": null,
2019-09-16T05:15:42.9259169Z       "suggestion_applicability": null,
2019-09-16T05:15:42.9259214Z       "expansion": null
2019-09-16T05:15:42.9259306Z   ],
2019-09-16T05:15:42.9259306Z   ],
2019-09-16T05:15:42.9259347Z   "children": [
2019-09-16T05:15:42.9259402Z     {
2019-09-16T05:15:42.9259460Z       "message": "possible candidates are found in other modules, you can import them into scope",
2019-09-16T05:15:42.9259509Z       "code": null,
2019-09-16T05:15:42.9259570Z       "level": "help",
2019-09-16T05:15:42.9259612Z       "spans": [
2019-09-16T05:15:42.9259715Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9259763Z           "byte_start": 448,
2019-09-16T05:15:42.9259807Z           "byte_end": 448,
2019-09-16T05:15:42.9259851Z           "line_start": 11,
2019-09-16T05:15:42.9259851Z           "line_start": 11,
2019-09-16T05:15:42.9259912Z           "line_end": 11,
2019-09-16T05:15:42.9259954Z           "column_start": 1,
2019-09-16T05:15:42.9259997Z           "column_end": 1,
2019-09-16T05:15:42.9260062Z           "is_primary": true,
2019-09-16T05:15:42.9260105Z           "text": [
2019-09-16T05:15:42.9260146Z             {
2019-09-16T05:15:42.9260205Z               "text": "fn main() {",
2019-09-16T05:15:42.9260250Z               "highlight_start": 1,
2019-09-16T05:15:42.9260295Z               "highlight_end": 1
2019-09-16T05:15:42.9260846Z           ],
2019-09-16T05:15:42.9260888Z           "label": null,
2019-09-16T05:15:42.9260888Z           "label": null,
2019-09-16T05:15:42.9261058Z           "suggested_replacement": "use std::collections::binary_heap::Iter;\n\n",
2019-09-16T05:15:42.9261125Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9261170Z           "expansion": null
2019-09-16T05:15:42.9261264Z         {
2019-09-16T05:15:42.9262821Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9262906Z           "byte_start": 448,
2019-09-16T05:15:42.9262969Z           "byte_end": 448,
2019-09-16T05:15:42.9262969Z           "byte_end": 448,
2019-09-16T05:15:42.9263167Z           "line_start": 11,
2019-09-16T05:15:42.9263210Z           "line_end": 11,
2019-09-16T05:15:42.9263252Z           "column_start": 1,
2019-09-16T05:15:42.9263311Z           "column_end": 1,
2019-09-16T05:15:42.9263354Z           "is_primary": true,
2019-09-16T05:15:42.9263396Z           "text": [
2019-09-16T05:15:42.9263460Z             {
2019-09-16T05:15:42.9263503Z               "text": "fn main() {",
2019-09-16T05:15:42.9263673Z               "highlight_start": 1,
2019-09-16T05:15:42.9263715Z               "highlight_end": 1
2019-09-16T05:15:42.9264564Z           ],
2019-09-16T05:15:42.9264606Z           "label": null,
2019-09-16T05:15:42.9264606Z           "label": null,
2019-09-16T05:15:42.9264678Z           "suggested_replacement": "use std::collections::btree_map::Iter;\n\n",
2019-09-16T05:15:42.9264753Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9264813Z           "expansion": null
2019-09-16T05:15:42.9264925Z         {
2019-09-16T05:15:42.9264971Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9265386Z           "byte_start": 448,
2019-09-16T05:15:42.9265440Z           "byte_end": 448,
2019-09-16T05:15:42.9265440Z           "byte_end": 448,
2019-09-16T05:15:42.9265483Z           "line_start": 11,
2019-09-16T05:15:42.9265527Z           "line_end": 11,
2019-09-16T05:15:42.9265686Z           "column_start": 1,
2019-09-16T05:15:42.9265739Z           "column_end": 1,
2019-09-16T05:15:42.9265782Z           "is_primary": true,
2019-09-16T05:15:42.9265838Z           "text": [
2019-09-16T05:15:42.9265880Z             {
2019-09-16T05:15:42.9265922Z               "text": "fn main() {",
2019-09-16T05:15:42.9265981Z               "highlight_start": 1,
2019-09-16T05:15:42.9266026Z               "highlight_end": 1
2019-09-16T05:15:42.9266108Z           ],
2019-09-16T05:15:42.9266165Z           "label": null,
2019-09-16T05:15:42.9266165Z           "label": null,
2019-09-16T05:15:42.9266214Z           "suggested_replacement": "use std::collections::btree_set::Iter;\n\n",
2019-09-16T05:15:42.9266272Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9266333Z           "expansion": null
2019-09-16T05:15:42.9266415Z         {
2019-09-16T05:15:42.9266477Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9266530Z           "byte_start": 448,
2019-09-16T05:15:42.9266572Z           "byte_end": 448,
2019-09-16T05:15:42.9266572Z           "byte_end": 448,
2019-09-16T05:15:42.9266615Z           "line_start": 11,
2019-09-16T05:15:42.9266673Z           "line_end": 11,
2019-09-16T05:15:42.9266714Z           "column_start": 1,
2019-09-16T05:15:42.9266756Z           "column_end": 1,
2019-09-16T05:15:42.9266815Z           "is_primary": true,
2019-09-16T05:15:42.9266857Z           "text": [
2019-09-16T05:15:42.9266897Z             {
2019-09-16T05:15:42.9266954Z               "text": "fn main() {",
2019-09-16T05:15:42.9266999Z               "highlight_start": 1,
2019-09-16T05:15:42.9267049Z               "highlight_end": 1
2019-09-16T05:15:42.9267145Z           ],
2019-09-16T05:15:42.9267186Z           "label": null,
2019-09-16T05:15:42.9267186Z           "label": null,
2019-09-16T05:15:42.9267234Z           "suggested_replacement": "use std::collections::hash_map::Iter;\n\n",
2019-09-16T05:15:42.9267506Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9267571Z           "expansion": null
2019-09-16T05:15:42.9267675Z         {
2019-09-16T05:15:42.9267723Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9267770Z           "byte_start": 448,
2019-09-16T05:15:42.9267828Z           "byte_end": 448,
2019-09-16T05:15:42.9267828Z           "byte_end": 448,
2019-09-16T05:15:42.9267873Z           "line_start": 11,
2019-09-16T05:15:42.9267915Z           "line_end": 11,
2019-09-16T05:15:42.9267957Z           "column_start": 1,
2019-09-16T05:15:42.9268017Z           "column_end": 1,
2019-09-16T05:15:42.9268713Z           "is_primary": true,
2019-09-16T05:15:42.9268871Z           "text": [
2019-09-16T05:15:42.9268930Z             {
2019-09-16T05:15:42.9268972Z               "text": "fn main() {",
2019-09-16T05:15:42.9269014Z               "highlight_start": 1,
2019-09-16T05:15:42.9269071Z               "highlight_end": 1
2019-09-16T05:15:42.9269150Z           ],
2019-09-16T05:15:42.9269198Z           "label": null,
2019-09-16T05:15:42.9269198Z           "label": null,
2019-09-16T05:15:42.9269260Z           "suggested_replacement": "use std::collections::hash_set::Iter;\n\n",
2019-09-16T05:15:42.9269309Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9269353Z           "expansion": null
2019-09-16T05:15:42.9269445Z         {
2019-09-16T05:15:42.9269490Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9269550Z           "byte_start": 448,
2019-09-16T05:15:42.9269591Z           "byte_end": 448,
2019-09-16T05:15:42.9269591Z           "byte_end": 448,
2019-09-16T05:15:42.9269631Z           "line_start": 11,
2019-09-16T05:15:42.9269679Z           "line_end": 11,
2019-09-16T05:15:42.9269735Z           "column_start": 1,
2019-09-16T05:15:42.9269776Z           "column_end": 1,
2019-09-16T05:15:42.9269817Z           "is_primary": true,
2019-09-16T05:15:42.9270336Z           "text": [
2019-09-16T05:15:42.9270404Z             {
2019-09-16T05:15:42.9270444Z               "text": "fn main() {",
2019-09-16T05:15:42.9270607Z               "highlight_start": 1,
2019-09-16T05:15:42.9270659Z               "highlight_end": 1
2019-09-16T05:15:42.9270736Z           ],
2019-09-16T05:15:42.9270789Z           "label": null,
2019-09-16T05:15:42.9270789Z           "label": null,
2019-09-16T05:15:42.9270973Z           "suggested_replacement": "use std::collections::linked_list::Iter;\n\n",
2019-09-16T05:15:42.9271859Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9271940Z           "expansion": null
2019-09-16T05:15:42.9272016Z         {
2019-09-16T05:15:42.9272075Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9272134Z           "byte_start": 448,
2019-09-16T05:15:42.9272174Z           "byte_end": 448,
2019-09-16T05:15:42.9272174Z           "byte_end": 448,
2019-09-16T05:15:42.9272213Z           "line_start": 11,
2019-09-16T05:15:42.9272376Z           "line_end": 11,
2019-09-16T05:15:42.9272413Z           "column_start": 1,
2019-09-16T05:15:42.9272456Z           "column_end": 1,
2019-09-16T05:15:42.9272509Z           "is_primary": true,
2019-09-16T05:15:42.9272546Z           "text": [
2019-09-16T05:15:42.9272582Z             {
2019-09-16T05:15:42.9272668Z               "text": "fn main() {",
2019-09-16T05:15:42.9272824Z               "highlight_start": 1,
2019-09-16T05:15:42.9272866Z               "highlight_end": 1
2019-09-16T05:15:42.9272959Z           ],
2019-09-16T05:15:42.9272997Z           "label": null,
2019-09-16T05:15:42.9272997Z           "label": null,
2019-09-16T05:15:42.9273043Z           "suggested_replacement": "use std::collections::vec_deque::Iter;\n\n",
2019-09-16T05:15:42.9273105Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9273154Z           "expansion": null
2019-09-16T05:15:42.9273245Z         {
2019-09-16T05:15:42.9273289Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9273332Z           "byte_start": 448,
2019-09-16T05:15:42.9273393Z           "byte_end": 448,
2019-09-16T05:15:42.9273393Z           "byte_end": 448,
2019-09-16T05:15:42.9273435Z           "line_start": 11,
2019-09-16T05:15:42.9273474Z           "line_end": 11,
2019-09-16T05:15:42.9273513Z           "column_start": 1,
2019-09-16T05:15:42.9273568Z           "column_end": 1,
2019-09-16T05:15:42.9273608Z           "is_primary": true,
2019-09-16T05:15:42.9273648Z           "text": [
2019-09-16T05:15:42.9273700Z             {
2019-09-16T05:15:42.9273741Z               "text": "fn main() {",
2019-09-16T05:15:42.9273899Z               "highlight_start": 1,
2019-09-16T05:15:42.9273957Z               "highlight_end": 1
2019-09-16T05:15:42.9274177Z           ],
2019-09-16T05:15:42.9274218Z           "label": null,
2019-09-16T05:15:42.9274218Z           "label": null,
2019-09-16T05:15:42.9274278Z           "suggested_replacement": "use std::option::Iter;\n\n",
2019-09-16T05:15:42.9274381Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9274542Z           "expansion": null
2019-09-16T05:15:42.9274646Z         {
2019-09-16T05:15:42.9274694Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9274793Z           "byte_start": 448,
2019-09-16T05:15:42.9274839Z           "byte_end": 448,
2019-09-16T05:15:42.9274839Z           "byte_end": 448,
2019-09-16T05:15:42.9274963Z           "line_start": 11,
2019-09-16T05:15:42.9275005Z           "line_end": 11,
2019-09-16T05:15:42.9275062Z           "column_start": 1,
2019-09-16T05:15:42.9275105Z           "column_end": 1,
2019-09-16T05:15:42.9275148Z           "is_primary": true,
2019-09-16T05:15:42.9275206Z           "text": [
2019-09-16T05:15:42.9275248Z             {
2019-09-16T05:15:42.9275291Z               "text": "fn main() {",
2019-09-16T05:15:42.9275359Z               "highlight_start": 1,
2019-09-16T05:15:42.9275402Z               "highlight_end": 1
2019-09-16T05:15:42.9275484Z           ],
2019-09-16T05:15:42.9275545Z           "label": null,
2019-09-16T05:15:42.9275545Z           "label": null,
2019-09-16T05:15:42.9275670Z           "suggested_replacement": "use std::path::Iter;\n\n",
2019-09-16T05:15:42.9275730Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9275791Z           "expansion": null
2019-09-16T05:15:42.9275873Z         {
2019-09-16T05:15:42.9275936Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9275984Z           "byte_start": 448,
2019-09-16T05:15:42.9276026Z           "byte_end": 448,
2019-09-16T05:15:42.9276026Z           "byte_end": 448,
2019-09-16T05:15:42.9276085Z           "line_start": 11,
2019-09-16T05:15:42.9276128Z           "line_end": 11,
2019-09-16T05:15:42.9276171Z           "column_start": 1,
2019-09-16T05:15:42.9276214Z           "column_end": 1,
2019-09-16T05:15:42.9276281Z           "is_primary": true,
2019-09-16T05:15:42.9276324Z           "text": [
2019-09-16T05:15:42.9276365Z             {
2019-09-16T05:15:42.9276423Z               "text": "fn main() {",
2019-09-16T05:15:42.9276468Z               "highlight_start": 1,
2019-09-16T05:15:42.9276512Z               "highlight_end": 1
2019-09-16T05:15:42.9276617Z           ],
2019-09-16T05:15:42.9276658Z           "label": null,
2019-09-16T05:15:42.9276658Z           "label": null,
2019-09-16T05:15:42.9276706Z           "suggested_replacement": "use std::result::Iter;\n\n",
2019-09-16T05:15:42.9276771Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9276817Z           "expansion": null
2019-09-16T05:15:42.9276913Z         {
2019-09-16T05:15:42.9276960Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9277007Z           "byte_start": 448,
2019-09-16T05:15:42.9277066Z           "byte_end": 448,
2019-09-16T05:15:42.9277066Z           "byte_end": 448,
2019-09-16T05:15:42.9277117Z           "line_start": 11,
2019-09-16T05:15:42.9277159Z           "line_end": 11,
2019-09-16T05:15:42.9277201Z           "column_start": 1,
2019-09-16T05:15:42.9335788Z           "column_end": 1,
2019-09-16T05:15:42.9335893Z           "is_primary": true,
2019-09-16T05:15:42.9335942Z           "text": [
2019-09-16T05:15:42.9336033Z             {
2019-09-16T05:15:42.9336081Z               "text": "fn main() {",
2019-09-16T05:15:42.9336129Z               "highlight_start": 1,
2019-09-16T05:15:42.9336192Z               "highlight_end": 1
2019-09-16T05:15:42.9336279Z           ],
2019-09-16T05:15:42.9336322Z           "label": null,
2019-09-16T05:15:42.9336322Z           "label": null,
2019-09-16T05:15:42.9336388Z           "suggested_replacement": "use std::slice::Iter;\n\n",
2019-09-16T05:15:42.9336439Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9336486Z           "expansion": null
2019-09-16T05:15:42.9336803Z         {
2019-09-16T05:15:42.9336852Z           "file_name": "/checkout/src/test/ui/lint/use_suggestion_json.rs",
2019-09-16T05:15:42.9336915Z           "byte_start": 448,
2019-09-16T05:15:42.9336960Z           "byte_end": 448,
2019-09-16T05:15:42.9336960Z           "byte_end": 448,
2019-09-16T05:15:42.9337004Z           "line_start": 11,
2019-09-16T05:15:42.9337047Z           "line_end": 11,
2019-09-16T05:15:42.9337115Z           "column_start": 1,
2019-09-16T05:15:42.9337159Z           "column_end": 1,
2019-09-16T05:15:42.9337203Z           "is_primary": true,
2019-09-16T05:15:42.9337262Z           "text": [
2019-09-16T05:15:42.9337304Z             {
2019-09-16T05:15:42.9337348Z               "text": "fn main() {",
2019-09-16T05:15:42.9337409Z               "highlight_start": 1,
2019-09-16T05:15:42.9337455Z               "highlight_end": 1
2019-09-16T05:15:42.9337538Z           ],
2019-09-16T05:15:42.9337597Z           "label": null,
2019-09-16T05:15:42.9337597Z           "label": null,
2019-09-16T05:15:42.9337646Z           "suggested_replacement": "use std::sync::mpsc::Iter;\n\n",
2019-09-16T05:15:42.9337703Z           "suggestion_applicability": "Unspecified",
2019-09-16T05:15:42.9337767Z           "expansion": null
2019-09-16T05:15:42.9337851Z       ],
2019-09-16T05:15:42.9337851Z       ],
2019-09-16T05:15:42.9337908Z       "children": [],
2019-09-16T05:15:42.9337950Z       "rendered": null
2019-09-16T05:15:42.9338119Z   ],
2019-09-16T05:15:42.9338119Z   ],
2019-09-16T05:15:42.9341871Z   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror[E0412]\u001b[0m\u001b[0m\u001b[1m: cannot find type `Iter` in this scope\u001b[0m\n\u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m/checkout/src/test/ui/lint/use_suggestion_json.rs:12:12\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: possible candidates are found in other modules, you can import them into scope\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::binary_heap::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_set::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::hash_map::Iter;\u001b[0m\n\u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m\n\u001b[0mand 8 other candidates\u001b[0m\n\n"
2019-09-16T05:15:42.9342309Z {
2019-09-16T05:15:42.9342309Z {
2019-09-16T05:15:42.9342360Z   "message": "aborting due to previous error",
2019-09-16T05:15:42.9342421Z   "code": null,
2019-09-16T05:15:42.9342469Z   "level": "error",
2019-09-16T05:15:42.9342511Z   "spans": [],
2019-09-16T05:15:42.9342567Z   "children": [],
2019-09-16T05:15:42.9342622Z   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m\n\n"
2019-09-16T05:15:42.9342728Z {
2019-09-16T05:15:42.9342728Z {
2019-09-16T05:15:42.9343142Z   "message": "For more information about this error, try `rustc --explain E0412`.",
2019-09-16T05:15:42.9343194Z   "code": null,
2019-09-16T05:15:42.9344191Z   "level": "failure-note",
2019-09-16T05:15:42.9344439Z   "spans": [],
2019-09-16T05:15:42.9344481Z   "children": [],
2019-09-16T05:15:42.9345206Z   "rendered": "\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m\n"
2019-09-16T05:15:42.9345901Z thread '[ui] ui/lint/use_suggestion_json.rs' panicked at 'explicit panic', src/tools/compiletest/src/json.rs:87:21
2019-09-16T05:15:42.9345977Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-16T05:15:42.9346032Z 
2019-09-16T05:15:42.9346057Z 
---
2019-09-16T05:15:42.9346731Z 
2019-09-16T05:15:42.9346990Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-16T05:15:42.9347088Z 
2019-09-16T05:15:42.9347116Z 
2019-09-16T05:15:42.9353890Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-16T05:15:42.9354174Z 
2019-09-16T05:15:42.9354220Z 
2019-09-16T05:15:42.9354267Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-16T05:15:42.9354318Z Build completed unsuccessfully in 1:11:35
2019-09-16T05:15:42.9354318Z Build completed unsuccessfully in 1:11:35
2019-09-16T05:15:42.9354543Z == clock drift check ==
2019-09-16T05:15:42.9369085Z   local time: Mon Sep 16 05:15:42 UTC 2019
2019-09-16T05:15:43.1006365Z   network time: Mon, 16 Sep 2019 05:15:43 GMT
2019-09-16T05:15:43.1006459Z == end clock drift check ==
2019-09-16T05:15:43.9872814Z ##[error]Bash exited with code '1'.
2019-09-16T05:15:43.9915685Z ##[section]Starting: Checkout
2019-09-16T05:15:43.9917686Z ==============================================================================
2019-09-16T05:15:43.9917741Z Task         : Get sources
2019-09-16T05:15:43.9917789Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
