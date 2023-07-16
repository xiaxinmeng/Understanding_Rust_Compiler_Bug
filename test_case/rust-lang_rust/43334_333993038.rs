
        # OS STRING
        if (unqualified_type_name == "OsString" and
            self.__conforms_to_field_layout(OS_STRING_FIELD_NAMES)):
            return TYPE_KIND_OS_STRING
