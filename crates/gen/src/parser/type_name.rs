use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum TypeName {
    HRESULT,
    HSTRING,
    I64,
    U64,
    Matrix3x2,
    Guid,
    Type,
    Const,
    IUnknown,
    IInspectable,
    IIterator,
    IIterable,
    IVectorView,
    IVector,
    IRestrictedErrorInfo,
    Other((&'static str, &'static str)),
}

const TABLE: [(&'static str, &'static str, TypeName); 13] = [
    ("Windows.Foundation", "HResult", TypeName::HRESULT),
    ("Windows.Win32.Com", "HRESULT", TypeName::HRESULT),
    ("Windows.Win32.WinRT", "HSTRING", TypeName::HSTRING),
    (
        "Windows.Win32.SystemServices",
        "LARGE_INTEGER",
        TypeName::I64,
    ),
    (
        "Windows.Win32.SystemServices",
        "ULARGE_INTEGER",
        TypeName::U64,
    ),
    (
        "Windows.Win32.Direct2D",
        "D2D_MATRIX_3X2_F",
        TypeName::Matrix3x2,
    ),
    ("System", "Guid", TypeName::Guid),
     ("Windows.Win32.Com", "Guid" , TypeName::Guid),
     ("System", "Type",  TypeName::Type),
     ("System.Runtime.CompilerServices", "IsConst", TypeName::Const),
    ("Windows.Win32.Com", "IUnknown", TypeName::IUnknown),
    (
        "Windows.Win32.WinRT",
        "IInspectable",
        TypeName::IInspectable,
    ),
    (
        "Windows.Foundation.Collections",
        "IIterator`1",
        TypeName::IIterator,
    ),
    (
        "Windows.Foundation.Collections",
        "IIterable`1",
        TypeName::IIterable,
    ),
    (
        "Windows.Foundation.Collections",
        "IVectorView`1",
        TypeName::IVectorView,
    ),
    (
        "Windows.Foundation.Collections",
        "IVector`1",
        TypeName::IVector,
    ),
    ("Windows.Win32.WinRT", "IRestrictedErrorInfo", TypeName::IRestrictedErrorInfo),
];

impl TypeName {
    pub fn from_full_name(full_name: (&'static str, &'static str)) -> Self {
        for (namespace, name, code) in &TABLE {
            if &full_name.1 == name && &full_name.0 == namespace {
                return *code;
            }
        }

        Self::Other(full_name)
    }

    pub fn to_full_name(&self) -> (&'static str, &'static str) {
        if let Self::Other(full_name) = self {
            return *full_name;
        }

        for (namespace, name, code) in &TABLE {
            if self == code {
                return (namespace, name);
            }
        }

        unexpected!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            TypeName::from_full_name(TypeName::IUnknown.to_full_name()),
            TypeName::IUnknown
        );
        assert_eq!(
            TypeName::from_full_name(TypeName::Other(("Namespace", "Name")).to_full_name()),
            TypeName::Other(("Namespace", "Name"))
        );
    }
}
