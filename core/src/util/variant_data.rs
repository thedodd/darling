use syn;

use {FromField, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantData<T> {
    Tuple(Vec<T>),
    Struct(Vec<T>),
    Unit,
}

impl<T> VariantData<T> {
    pub fn empty_from(src: &syn::VariantData) -> Self {
        match *src {
            syn::VariantData::Struct(_) => VariantData::Struct(vec![]),
            syn::VariantData::Tuple(_) => VariantData::Tuple(vec![]),
            syn::VariantData::Unit => VariantData::Unit,
        }
    }

    /// Gets all field declarations. Returns an empty `Vec` for `VariantData::Unit`.
    pub fn fields<'a>(&'a self) -> Vec<&'a T> {
        match *self {
            VariantData::Tuple(ref fields) |
            VariantData::Struct(ref fields) => fields.iter().collect(),
            VariantData::Unit => Vec::new(),
        }
    }

    pub fn map<F, U>(self, map: F) -> VariantData<U>
        where F: FnMut(T) -> U
    {
        match self {
            VariantData::Tuple(fields) => VariantData::Tuple(fields.into_iter().map(map).collect()),
            VariantData::Struct(fields) => {
                VariantData::Struct(fields.into_iter().map(map).collect())
            }
            VariantData::Unit => VariantData::Unit,
        }
    }

    pub fn as_ref<'a>(&'a self) -> VariantData<&'a T> {
        match *self {
            VariantData::Tuple(ref fields) => VariantData::Tuple(fields.iter().collect()),
            VariantData::Struct(ref fields) => VariantData::Struct(fields.iter().collect()),
            VariantData::Unit => VariantData::Unit,
        }
    }
}

impl<F: FromField> VariantData<F> {
    pub fn from(vd: &syn::VariantData) -> Result<Self> {
        match *vd {
            syn::VariantData::Unit => Ok(VariantData::Unit),
            syn::VariantData::Tuple(ref fields) => {
                let mut f = Vec::with_capacity(fields.len());
                for field in fields {
                    f.push(FromField::from_field(field)?);
                }

                Ok(VariantData::Tuple(f))
            }
            syn::VariantData::Struct(ref fields) => {
                let mut f = Vec::with_capacity(fields.len());
                for field in fields {
                    f.push(FromField::from_field(field)?);
                }

                Ok(VariantData::Struct(f))
            }
        }
    }
}