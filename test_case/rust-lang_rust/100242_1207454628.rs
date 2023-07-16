json
"0:9614:1703": {
            "id": "0:9614:1703",
            "crate_id": 0,
            "name": "Mat3",
            "span": {
                "filename": "/home/makspll/.cargo/registry/src/github.com-1ecc6299db9ec823/glam-0.21.2/src/f32/mat3.rs",
                "begin": [
                    45,
                    0
                ],
                "end": [
                    49,
                    1
                ]
            },
            "visibility": "public",
            "docs": "A 3x3 column major matrix.\n\nThis 3x3 matrix type features convenience methods for creating and using linear and\naffine transformations. If you are primarily dealing with 2D affine transformations the\n[`Affine2`](crate::Affine2) type is much faster and more space efficient than\nusing a 3x3 matrix.\n\nLinear transformations including 3D rotation and scale can be created using methods\nsuch as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],\n[`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or\n[`Self::from_rotation_z()`].\n\nThe resulting matrices can be use to transform 3D vectors using regular vector\nmultiplication.\n\nAffine transformations including 2D translation, rotation and scale can be created\nusing methods such as [`Self::from_translation()`], [`Self::from_angle()`],\n[`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].\n\nThe [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods\nare provided for performing affine transforms on 2D vectors and points. These multiply\n2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for\nvectors respectively. These methods assume that `Self` contains a valid affine\ntransform.",
            "links": {
                "`Self::transform_vector2()`": "0:9614:1703",
                "`Self::from_quat()`": "0:9614:1703",
                "`Self::from_rotation_x()`": "0:9614:1703",
                "`Self::from_rotation_z()`": "0:9614:1703",
                "`Self::from_scale()`": "0:9614:1703",
                "crate::Affine2": "0:9602:2017",
                "`Self::transform_point2()`": "0:9614:1703",
                "`Self::from_rotation_y()`": "0:9614:1703",
                "`Self::from_angle()`": "0:9614:1703",
                "`Self::from_diagonal()`": "0:9614:1703",
                "`Self::from_translation()`": "0:9614:1703",
                "`Self::from_scale_angle_translation()`": "0:9614:1703",
                "`Self::from_axis_angle()`": "0:9614:1703"
            },
            "attrs": [
                "#[repr(C)]"
            ],
            "deprecation": null,
            "kind": "struct",
            "inner": {
                "struct_type": "plain",
                "generics": {
                    "params": [],
                    "where_predicates": []
                },
                "fields_stripped": false,
                "fields": [
                    "0:9615:1779",
                    "0:9616:1780",
                    "0:9617:1782"
                ],
                "impls": [
                    "0:1075",
                    "a:2:9217:9876-0:9614:1703",
                    "a:2:3276:4385-0:9614:1703",
                    "a:2:3247:208-0:9614:1703",
                    "a:2:9218:9875-0:9614:1703",
                    "a:2:3235:199-0:9614:1703",
                    "b:2:3179-0:9614:1703",
                    "b:2:3168-0:9614:1703",
                    "b:2:3164-0:9614:1703",
                    "b:2:2838-0:9614:1703",
                    "b:2:3964-0:9614:1703",
                    "b:2:3174-0:9614:1703",
                    "b:2:2841-0:9614:1703",
                    "b:5:6398-0:9614:1703",
                    "b:5:779-0:9614:1703",
                    "b:20:93-0:9614:1703",
                    "b:20:425-0:9614:1703",
                    "b:20:114-0:9614:1703",
                    "b:22:1583-0:9614:1703",
                    "0:138",
                    "0:139",
                    "0:9150",
                    "0:9153",
                    "0:878",
                    "0:880",
                    "0:883",
                    "0:9618",
                    "0:9620",
                    "0:1124",
                    "0:1126",
                    "0:1129",
                    "0:1131",
                    "0:1134",
                    "0:1136",
                    "0:1139",
                    "0:1142",
                    "0:1144",
                    "0:1147",
                    "0:1150",
                    "0:1153",
                    "0:1155",
                    "0:1158",
                    "0:1160",
                    "0:1165",
                    "0:1170",
                    "0:1172",
                    "0:1177",
                    "0:1182",
                    "0:1184",
                    "0:1924"
                ]
            }
        },
