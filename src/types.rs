use std::fmt;
use error::{Result, Error};

/// Data structure representing the `proto_id_item` type.
#[derive(Debug, Clone)]
pub struct PrototypeIdData {
    shorty_index: u32,
    return_type_index: u32,
    parameters_offset: Option<u32>,
}

impl PrototypeIdData {
    /// Creates a new `PrototypeIdData` from the three `u32` that conform the `proto_id_item` type.
    pub fn new(shorty_index: u32,
               return_type_index: u32,
               parameters_offset: u32)
               -> PrototypeIdData {
        PrototypeIdData {
            shorty_index: shorty_index,
            return_type_index: return_type_index,
            parameters_offset: if parameters_offset != 0 {
                Some(parameters_offset)
            } else {
                None
            },
        }
    }

    /// Gets the shorty index.
    ///
    /// Gets the index into the `string_ids` list for the short-form descriptor string of this
    /// prototype. The string must conform to the syntax for `ShortyDescriptor`, and must
    /// correspond to the return type and parameters of this item.
    pub fn get_shorty_index(&self) -> usize {
        self.shorty_index as usize
    }

    /// Gets the return type index.
    ///
    /// Gets the index into the `type_ids` list for the return type of this prototype.
    pub fn get_return_type_index(&self) -> usize {
        self.return_type_index as usize
    }

    /// Gets the parameter list offset, if the prototype has parameters.
    ///
    /// Gets the offset from the start of the file to the list of parameter types for this
    /// prototype, or `None` if this prototype has no parameters. This offset, should be in the
    /// data section, and the `data` there should be in the format specified by `type_list`.
    /// Additionally, there should be no reference to the type `void` in the list.
    pub fn get_parameters_offset(&self) -> Option<u32> {
        self.parameters_offset
    }
}

/// Structure representing the `field_id_item` type.
#[derive(Debug, Clone)]
pub struct FieldIdData {
    class_index: u16,
    type_index: u16,
    name_index: u32,
}

impl FieldIdData {
    /// Creates a new `FieldIdData` from the two `u16` and the `u32` that conform the
    /// `field_id_item` type.
    pub fn new(class_index: u16, type_index: u16, name_index: u32) -> FieldIdData {
        FieldIdData {
            class_index: class_index,
            type_index: type_index,
            name_index: name_index,
        }
    }

    /// Gets the index of the class of the field.
    ///
    /// Gets the index into the `type_ids` list for the definer of this field. This must be a class
    /// type, and not an array or primitive type.
    pub fn get_class_index(&self) -> usize {
        self.class_index as usize
    }

    /// Gets the index of the type of the class.
    ///
    /// Gets the index into the `type_ids` list for the type of this field.
    pub fn get_type_index(&self) -> usize {
        self.type_index as usize
    }

    /// Gets the index to the name of this field.
    ///
    /// Gets the index into the `string_ids` list for the name of this field. The string must
    /// conform to the syntax for `MemberName`.
    pub fn get_name_index(&self) -> usize {
        self.name_index as usize
    }
}

/// Structure representing the `method_id_item` type.
#[derive(Debug, Clone)]
pub struct MethodIdData {
    class_index: u16,
    prototype_index: u16,
    name_index: u32,
}

impl MethodIdData {
    /// Creates a new `MethodIdData` from the two `u16` and the `u32` that conform the
    /// `method_id_item` type.
    pub fn new(class_index: u16, prototype_index: u16, name_index: u32) -> MethodIdData {
        MethodIdData {
            class_index: class_index,
            prototype_index: prototype_index,
            name_index: name_index,
        }
    }

    /// Gets the index of the class of the field.
    ///
    /// Gets the index into the `type_ids` list for the definer of this method. This must be a
    /// class or array type, and not a primitive type.
    pub fn get_class_index(&self) -> usize {
        self.class_index as usize
    }

    /// Gets the index of the prototype of the class.
    ///
    /// Gets the index into the `prototype_ids` list for the prototype of this method.
    pub fn get_prototype_index(&self) -> usize {
        self.prototype_index as usize
    }

    /// Gets the index to the name of this field.
    ///
    /// Gets the index into the `string_ids` list for the name of this field. The string must
    /// conform to the syntax for `MemberName`.
    pub fn get_name_index(&self) -> usize {
        self.name_index as usize
    }
}

const NO_INDEX: u32 = 0xFFFFFFFF;

bitflags! {
    flags AccessFlags: u32 {
        const ACC_PUBLIC = 0x1,
        const ACC_PRIVATE = 0x2,
        const ACC_PROTECTED = 0x4,
        const ACC_STATIC = 0x8,
        const ACC_FINAL = 0x10,
        const ACC_SYNCHRONIZED = 0x20,
        const ACC_VOLATILE = 0x40,
        const ACC_BRIDGE = 0x40,
        const ACC_TRANSIENT = 0x80,
        const ACC_VARARGS = 0x80,
        const ACC_NATIVE = 0x100,
        const ACC_INTERFACE = 0x200,
        const ACC_ABSTRACT = 0x400,
        const ACC_STRICT = 0x800,
        const ACC_SYNTHETIC = 0x1000,
        const ACC_ANNOTATION = 0x2000,
        const ACC_ENUM = 0x4000,
        const ACC_CONSTRUCTOR = 0x10000,
        const ACC_DECLARED_SYNCHRONIZED = 0x20000,
    }
}

pub struct ClassDefData {
    class_id: u32,
    access_flags: AccessFlags,
    superclass_id: Option<u32>,
    interfaces_offset: Option<u32>,
    source_file_id: Option<u32>,
    annotations_offset: Option<u32>,
    class_data_offset: Option<u32>,
    static_values_offset: Option<u32>,
}

impl ClassDefData {
    pub fn new(class_id: u32,
               access_flags: u32,
               superclass_id: u32,
               interfaces_offset: u32,
               source_file_id: u32,
               annotations_offset: u32,
               class_data_offset: u32,
               static_values_offset: u32)
               -> Result<ClassDefData> {
        Ok(ClassDefData {
            class_id: class_id,
            access_flags: try!(AccessFlags::from_bits(access_flags)
                .ok_or(Error::invalid_access_flags(access_flags))),
            superclass_id: if superclass_id != NO_INDEX {
                Some(superclass_id)
            } else {
                None
            },
            interfaces_offset: if interfaces_offset != 0 {
                Some(interfaces_offset)
            } else {
                None
            },
            source_file_id: if source_file_id != NO_INDEX {
                Some(source_file_id)
            } else {
                None
            },
            annotations_offset: if annotations_offset != 0 {
                Some(annotations_offset)
            } else {
                None
            },
            class_data_offset: if class_data_offset != 0 {
                Some(class_data_offset)
            } else {
                None
            },
            static_values_offset: if static_values_offset != 0 {
                Some(static_values_offset)
            } else {
                None
            },
        })
    }
}

const TYPE_HEADER_ITEM: u16 = 0x0000;
const TYPE_STRING_ID_ITEM: u16 = 0x0001;
const TYPE_TYPE_ID_ITEM: u16 = 0x0002;
const TYPE_PROTO_ID_ITEM: u16 = 0x0003;
const TYPE_FIELD_ID_ITEM: u16 = 0x0004;
const TYPE_METHOD_ID_ITEM: u16 = 0x0005;
const TYPE_CLASS_DEF_ITEM: u16 = 0x0006;
const TYPE_MAP_LIST: u16 = 0x1000;
const TYPE_TYPE_LIST: u16 = 0x1001;
const TYPE_ANNOTATION_SET_REF_LIST: u16 = 0x1002;
const TYPE_ANNOTATION_SET_ITEM: u16 = 0x1003;
const TYPE_CLASS_DATA_ITEM: u16 = 0x2000;
const TYPE_CODE_ITEM: u16 = 0x2001;
const TYPE_STRING_DATA_ITEM: u16 = 0x2002;
const TYPE_DEBUG_INFO_ITEM: u16 = 0x2003;
const TYPE_ANNOTATION_ITEM: u16 = 0x2004;
const TYPE_ENCODED_ARRAY_ITEM: u16 = 0x2005;
const TYPE_ANNOTATIONS_DIRECTORY_ITEM: u16 = 0x2006;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemType {
    Header,
    StringId,
    TypeId,
    ProtoId,
    FieldId,
    MethodId,
    ClassDef,
    Map,
    TypeList,
    AnnotationSetList,
    AnnotationSet,
    ClassData,
    Code,
    StringData,
    DebugInfo,
    Annotation,
    EncodedArray,
    AnnotationsDirectory,
}

impl ItemType {
    fn from_u16(value: u16) -> Result<ItemType> {
        match value {
            TYPE_HEADER_ITEM => Ok(ItemType::Header),
            TYPE_STRING_ID_ITEM => Ok(ItemType::StringId),
            TYPE_TYPE_ID_ITEM => Ok(ItemType::TypeId),
            TYPE_PROTO_ID_ITEM => Ok(ItemType::ProtoId),
            TYPE_FIELD_ID_ITEM => Ok(ItemType::FieldId),
            TYPE_METHOD_ID_ITEM => Ok(ItemType::MethodId),
            TYPE_CLASS_DEF_ITEM => Ok(ItemType::ClassDef),
            TYPE_MAP_LIST => Ok(ItemType::Map),
            TYPE_TYPE_LIST => Ok(ItemType::TypeList),
            TYPE_ANNOTATION_SET_REF_LIST => Ok(ItemType::AnnotationSetList),
            TYPE_ANNOTATION_SET_ITEM => Ok(ItemType::AnnotationSet),
            TYPE_CLASS_DATA_ITEM => Ok(ItemType::ClassData),
            TYPE_CODE_ITEM => Ok(ItemType::Code),
            TYPE_STRING_DATA_ITEM => Ok(ItemType::StringData),
            TYPE_DEBUG_INFO_ITEM => Ok(ItemType::DebugInfo),
            TYPE_ANNOTATION_ITEM => Ok(ItemType::Annotation),
            TYPE_ENCODED_ARRAY_ITEM => Ok(ItemType::EncodedArray),
            TYPE_ANNOTATIONS_DIRECTORY_ITEM => Ok(ItemType::AnnotationsDirectory),
            v => Err(Error::invalid_item_type(v)),
        }
    }
}

impl From<ItemType> for u16 {
    fn from(item_type: ItemType) -> u16 {
        match item_type {
            ItemType::Header => TYPE_HEADER_ITEM,
            ItemType::StringId => TYPE_STRING_ID_ITEM,
            ItemType::TypeId => TYPE_TYPE_ID_ITEM,
            ItemType::ProtoId => TYPE_PROTO_ID_ITEM,
            ItemType::FieldId => TYPE_FIELD_ID_ITEM,
            ItemType::MethodId => TYPE_METHOD_ID_ITEM,
            ItemType::ClassDef => TYPE_CLASS_DEF_ITEM,
            ItemType::Map => TYPE_MAP_LIST,
            ItemType::TypeList => TYPE_TYPE_LIST,
            ItemType::AnnotationSetList => TYPE_ANNOTATION_SET_REF_LIST,
            ItemType::AnnotationSet => TYPE_ANNOTATION_SET_ITEM,
            ItemType::ClassData => TYPE_CLASS_DATA_ITEM,
            ItemType::Code => TYPE_CODE_ITEM,
            ItemType::StringData => TYPE_STRING_DATA_ITEM,
            ItemType::DebugInfo => TYPE_DEBUG_INFO_ITEM,
            ItemType::Annotation => TYPE_ANNOTATION_ITEM,
            ItemType::EncodedArray => TYPE_ENCODED_ARRAY_ITEM,
            ItemType::AnnotationsDirectory => TYPE_ANNOTATIONS_DIRECTORY_ITEM,
        }
    }
}

pub struct MapItem {
    item_type: ItemType,
    size: u32,
    offset: u32,
}

impl fmt::Debug for MapItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "MapItem {{ item_type: {:?} ({:#06x}), size: {} items, offset: {:#010x} }}",
               self.item_type,
               u16::from(self.item_type),
               self.size,
               self.offset)
    }
}

impl MapItem {
    pub fn new(item_type: u16, size: u32, offset: u32) -> Result<MapItem> {
        Ok(MapItem {
            item_type: try!(ItemType::from_u16(item_type)),
            size: size,
            offset: offset,
        })
    }

    pub fn get_item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn get_num_items(&self) -> usize {
        self.size as usize
    }

    pub fn get_offset(&self) -> usize {
        self.offset as usize
    }
}
