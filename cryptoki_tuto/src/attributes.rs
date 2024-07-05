use cryptoki::object::AttributeType;

pub fn get_attribute_types() -> Vec<AttributeType> {
    let attribute_types: Vec<AttributeType> = vec![
        AttributeType::AcIssuer,
        // AttributeType::AllowedMechanisms,
        AttributeType::AlwaysAuthenticate,
        AttributeType::AlwaysSensitive,
        AttributeType::Application,
        AttributeType::AttrTypes,
        AttributeType::Base,
        AttributeType::CertificateType,
        AttributeType::CheckValue,
        AttributeType::Class,
        AttributeType::Coefficient,
        AttributeType::Copyable,
        AttributeType::Decrypt,
        AttributeType::Derive,
        AttributeType::Destroyable,
        AttributeType::EcParams,
        AttributeType::EcPoint,
        AttributeType::Encrypt,
        AttributeType::EndDate,
        AttributeType::Exponent1,
        AttributeType::Exponent2,
        AttributeType::Extractable,
        AttributeType::HashOfIssuerPublicKey,
        AttributeType::HashOfSubjectPublicKey,
        AttributeType::Id,
        AttributeType::Issuer,
        AttributeType::KeyGenMechanism,
        AttributeType::KeyType,
        AttributeType::Label,
        AttributeType::Local,
        AttributeType::Modifiable,
        AttributeType::Modulus,
        AttributeType::ModulusBits,
        AttributeType::NeverExtractable,
        AttributeType::ObjectId,
        AttributeType::Owner,
        AttributeType::Prime,
        AttributeType::Prime1,
        AttributeType::Prime2,
        AttributeType::Private,
        AttributeType::PrivateExponent,
        AttributeType::PublicExponent,
        AttributeType::PublicKeyInfo,
        AttributeType::Sensitive,
        AttributeType::SerialNumber,
        AttributeType::Sign,
        AttributeType::SignRecover,
        AttributeType::StartDate,
        AttributeType::Subject,
        AttributeType::Token,
        AttributeType::Trusted,
        AttributeType::Unwrap,
        AttributeType::Url,
        AttributeType::Value,
        AttributeType::ValueLen,
        AttributeType::Verify,
        AttributeType::VerifyRecover,
        AttributeType::Wrap,
        AttributeType::WrapWithTrusted,
    ];
    attribute_types
}