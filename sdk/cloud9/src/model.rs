// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Information about an environment member for an Cloud9 development environment.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct EnvironmentMember {
    /// <p>The type of environment member permissions associated with this environment member.
    /// Available values include:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>owner</code>: Owns the environment.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>read-only</code>: Has read-only access to the environment.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>read-write</code>: Has read-write access to the environment.</p>
    /// </li>
    /// </ul>
    pub permissions: std::option::Option<crate::model::Permissions>,
    /// <p>The user ID in Identity and Access Management (IAM) of the environment member.</p>
    pub user_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the environment member.</p>
    pub user_arn: std::option::Option<std::string::String>,
    /// <p>The ID of the environment for the environment member.</p>
    pub environment_id: std::option::Option<std::string::String>,
    /// <p>The time, expressed in epoch time format, when the environment member last opened the
    /// environment.</p>
    pub last_access: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for EnvironmentMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EnvironmentMember");
        formatter.field("permissions", &self.permissions);
        formatter.field("user_id", &self.user_id);
        formatter.field("user_arn", &self.user_arn);
        formatter.field("environment_id", &self.environment_id);
        formatter.field("last_access", &self.last_access);
        formatter.finish()
    }
}
/// See [`EnvironmentMember`](crate::model::EnvironmentMember)
pub mod environment_member {
    /// A builder for [`EnvironmentMember`](crate::model::EnvironmentMember)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) permissions: std::option::Option<crate::model::Permissions>,
        pub(crate) user_id: std::option::Option<std::string::String>,
        pub(crate) user_arn: std::option::Option<std::string::String>,
        pub(crate) environment_id: std::option::Option<std::string::String>,
        pub(crate) last_access: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The type of environment member permissions associated with this environment member.
        /// Available values include:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>owner</code>: Owns the environment.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>read-only</code>: Has read-only access to the environment.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>read-write</code>: Has read-write access to the environment.</p>
        /// </li>
        /// </ul>
        pub fn permissions(mut self, input: crate::model::Permissions) -> Self {
            self.permissions = Some(input);
            self
        }
        pub fn set_permissions(
            mut self,
            input: std::option::Option<crate::model::Permissions>,
        ) -> Self {
            self.permissions = input;
            self
        }
        /// <p>The user ID in Identity and Access Management (IAM) of the environment member.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the environment member.</p>
        pub fn user_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_arn = Some(input.into());
            self
        }
        pub fn set_user_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_arn = input;
            self
        }
        /// <p>The ID of the environment for the environment member.</p>
        pub fn environment_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_id = Some(input.into());
            self
        }
        pub fn set_environment_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.environment_id = input;
            self
        }
        /// <p>The time, expressed in epoch time format, when the environment member last opened the
        /// environment.</p>
        pub fn last_access(mut self, input: smithy_types::Instant) -> Self {
            self.last_access = Some(input);
            self
        }
        pub fn set_last_access(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_access = input;
            self
        }
        /// Consumes the builder and constructs a [`EnvironmentMember`](crate::model::EnvironmentMember)
        pub fn build(self) -> crate::model::EnvironmentMember {
            crate::model::EnvironmentMember {
                permissions: self.permissions,
                user_id: self.user_id,
                user_arn: self.user_arn,
                environment_id: self.environment_id,
                last_access: self.last_access,
            }
        }
    }
}
impl EnvironmentMember {
    /// Creates a new builder-style object to manufacture [`EnvironmentMember`](crate::model::EnvironmentMember)
    pub fn builder() -> crate::model::environment_member::Builder {
        crate::model::environment_member::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum Permissions {
    Owner,
    ReadOnly,
    ReadWrite,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for Permissions {
    fn from(s: &str) -> Self {
        match s {
            "owner" => Permissions::Owner,
            "read-only" => Permissions::ReadOnly,
            "read-write" => Permissions::ReadWrite,
            other => Permissions::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for Permissions {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Permissions::from(s))
    }
}
impl Permissions {
    pub fn as_str(&self) -> &str {
        match self {
            Permissions::Owner => "owner",
            Permissions::ReadOnly => "read-only",
            Permissions::ReadWrite => "read-write",
            Permissions::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["owner", "read-only", "read-write"]
    }
}
impl AsRef<str> for Permissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum MemberPermissions {
    ReadOnly,
    ReadWrite,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for MemberPermissions {
    fn from(s: &str) -> Self {
        match s {
            "read-only" => MemberPermissions::ReadOnly,
            "read-write" => MemberPermissions::ReadWrite,
            other => MemberPermissions::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for MemberPermissions {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(MemberPermissions::from(s))
    }
}
impl MemberPermissions {
    pub fn as_str(&self) -> &str {
        match self {
            MemberPermissions::ReadOnly => "read-only",
            MemberPermissions::ReadWrite => "read-write",
            MemberPermissions::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["read-only", "read-write"]
    }
}
impl AsRef<str> for MemberPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ManagedCredentialsAction {
    Disable,
    Enable,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ManagedCredentialsAction {
    fn from(s: &str) -> Self {
        match s {
            "DISABLE" => ManagedCredentialsAction::Disable,
            "ENABLE" => ManagedCredentialsAction::Enable,
            other => ManagedCredentialsAction::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ManagedCredentialsAction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ManagedCredentialsAction::from(s))
    }
}
impl ManagedCredentialsAction {
    pub fn as_str(&self) -> &str {
        match self {
            ManagedCredentialsAction::Disable => "DISABLE",
            ManagedCredentialsAction::Enable => "ENABLE",
            ManagedCredentialsAction::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["DISABLE", "ENABLE"]
    }
}
impl AsRef<str> for ManagedCredentialsAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Metadata that is associated with Amazon Web Services resources. In particular, a name-value pair that
/// can be associated with an Cloud9 development environment. There are two types of tags:
/// <i>user tags</i> and <i>system tags</i>. A user tag is created
/// by the user. A system tag is automatically created by Amazon Web Services services. A system tag is prefixed
/// with <code>"aws:"</code> and cannot be modified by the user.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The <b>name</b> part of a tag.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The <b>value</b> part of a tag.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &"*** Sensitive Data Redacted ***");
        formatter.field("value", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The <b>name</b> part of a tag.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The <b>value</b> part of a tag.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum EnvironmentStatus {
    Connecting,
    Creating,
    Deleting,
    Error,
    Ready,
    Stopped,
    Stopping,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for EnvironmentStatus {
    fn from(s: &str) -> Self {
        match s {
            "connecting" => EnvironmentStatus::Connecting,
            "creating" => EnvironmentStatus::Creating,
            "deleting" => EnvironmentStatus::Deleting,
            "error" => EnvironmentStatus::Error,
            "ready" => EnvironmentStatus::Ready,
            "stopped" => EnvironmentStatus::Stopped,
            "stopping" => EnvironmentStatus::Stopping,
            other => EnvironmentStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for EnvironmentStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(EnvironmentStatus::from(s))
    }
}
impl EnvironmentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            EnvironmentStatus::Connecting => "connecting",
            EnvironmentStatus::Creating => "creating",
            EnvironmentStatus::Deleting => "deleting",
            EnvironmentStatus::Error => "error",
            EnvironmentStatus::Ready => "ready",
            EnvironmentStatus::Stopped => "stopped",
            EnvironmentStatus::Stopping => "stopping",
            EnvironmentStatus::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "connecting",
            "creating",
            "deleting",
            "error",
            "ready",
            "stopped",
            "stopping",
        ]
    }
}
impl AsRef<str> for EnvironmentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about an Cloud9 development environment.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Environment {
    /// <p>The ID of the environment.</p>
    pub id: std::option::Option<std::string::String>,
    /// <p>The name of the environment.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>The description for the environment.</p>
    pub description: std::option::Option<std::string::String>,
    /// <p>The type of environment. Valid values include the following:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>ec2</code>: An Amazon Elastic Compute Cloud (Amazon EC2) instance connects to the environment.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ssh</code>: Your own server connects to the environment.</p>
    /// </li>
    /// </ul>
    pub r#type: std::option::Option<crate::model::EnvironmentType>,
    /// <p>The connection type used for connecting to an Amazon EC2 environment. <code>CONNECT_SSH</code>
    /// is selected by default.</p>
    pub connection_type: std::option::Option<crate::model::ConnectionType>,
    /// <p>The Amazon Resource Name (ARN) of the environment.</p>
    pub arn: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the environment owner.</p>
    pub owner_arn: std::option::Option<std::string::String>,
    /// <p>The state of the environment in its creation or deletion lifecycle.</p>
    pub lifecycle: std::option::Option<crate::model::EnvironmentLifecycle>,
    /// <p>Describes the status of Amazon Web Services managed temporary credentials for the Cloud9 environment.
    /// Available values are:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>ENABLED_ON_CREATE</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ENABLED_BY_OWNER</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>DISABLED_BY_DEFAULT</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>DISABLED_BY_OWNER</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>DISABLED_BY_COLLABORATOR</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>PENDING_REMOVAL_BY_COLLABORATOR</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>PENDING_REMOVAL_BY_OWNER</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>FAILED_REMOVAL_BY_COLLABORATOR</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ENABLED_BY_OWNER</code>
    /// </p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>DISABLED_BY_DEFAULT</code>
    /// </p>
    /// </li>
    /// </ul>
    pub managed_credentials_status: std::option::Option<crate::model::ManagedCredentialsStatus>,
}
impl std::fmt::Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Environment");
        formatter.field("id", &self.id);
        formatter.field("name", &self.name);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("r#type", &self.r#type);
        formatter.field("connection_type", &self.connection_type);
        formatter.field("arn", &self.arn);
        formatter.field("owner_arn", &self.owner_arn);
        formatter.field("lifecycle", &self.lifecycle);
        formatter.field(
            "managed_credentials_status",
            &self.managed_credentials_status,
        );
        formatter.finish()
    }
}
/// See [`Environment`](crate::model::Environment)
pub mod environment {
    /// A builder for [`Environment`](crate::model::Environment)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) id: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) r#type: std::option::Option<crate::model::EnvironmentType>,
        pub(crate) connection_type: std::option::Option<crate::model::ConnectionType>,
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) owner_arn: std::option::Option<std::string::String>,
        pub(crate) lifecycle: std::option::Option<crate::model::EnvironmentLifecycle>,
        pub(crate) managed_credentials_status:
            std::option::Option<crate::model::ManagedCredentialsStatus>,
    }
    impl Builder {
        /// <p>The ID of the environment.</p>
        pub fn id(mut self, input: impl Into<std::string::String>) -> Self {
            self.id = Some(input.into());
            self
        }
        pub fn set_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.id = input;
            self
        }
        /// <p>The name of the environment.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>The description for the environment.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// <p>The type of environment. Valid values include the following:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>ec2</code>: An Amazon Elastic Compute Cloud (Amazon EC2) instance connects to the environment.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>ssh</code>: Your own server connects to the environment.</p>
        /// </li>
        /// </ul>
        pub fn r#type(mut self, input: crate::model::EnvironmentType) -> Self {
            self.r#type = Some(input);
            self
        }
        pub fn set_type(
            mut self,
            input: std::option::Option<crate::model::EnvironmentType>,
        ) -> Self {
            self.r#type = input;
            self
        }
        /// <p>The connection type used for connecting to an Amazon EC2 environment. <code>CONNECT_SSH</code>
        /// is selected by default.</p>
        pub fn connection_type(mut self, input: crate::model::ConnectionType) -> Self {
            self.connection_type = Some(input);
            self
        }
        pub fn set_connection_type(
            mut self,
            input: std::option::Option<crate::model::ConnectionType>,
        ) -> Self {
            self.connection_type = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the environment.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the environment owner.</p>
        pub fn owner_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.owner_arn = Some(input.into());
            self
        }
        pub fn set_owner_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.owner_arn = input;
            self
        }
        /// <p>The state of the environment in its creation or deletion lifecycle.</p>
        pub fn lifecycle(mut self, input: crate::model::EnvironmentLifecycle) -> Self {
            self.lifecycle = Some(input);
            self
        }
        pub fn set_lifecycle(
            mut self,
            input: std::option::Option<crate::model::EnvironmentLifecycle>,
        ) -> Self {
            self.lifecycle = input;
            self
        }
        /// <p>Describes the status of Amazon Web Services managed temporary credentials for the Cloud9 environment.
        /// Available values are:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>ENABLED_ON_CREATE</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>ENABLED_BY_OWNER</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>DISABLED_BY_DEFAULT</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>DISABLED_BY_OWNER</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>DISABLED_BY_COLLABORATOR</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>PENDING_REMOVAL_BY_COLLABORATOR</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>PENDING_REMOVAL_BY_OWNER</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>FAILED_REMOVAL_BY_COLLABORATOR</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>ENABLED_BY_OWNER</code>
        /// </p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>DISABLED_BY_DEFAULT</code>
        /// </p>
        /// </li>
        /// </ul>
        pub fn managed_credentials_status(
            mut self,
            input: crate::model::ManagedCredentialsStatus,
        ) -> Self {
            self.managed_credentials_status = Some(input);
            self
        }
        pub fn set_managed_credentials_status(
            mut self,
            input: std::option::Option<crate::model::ManagedCredentialsStatus>,
        ) -> Self {
            self.managed_credentials_status = input;
            self
        }
        /// Consumes the builder and constructs a [`Environment`](crate::model::Environment)
        pub fn build(self) -> crate::model::Environment {
            crate::model::Environment {
                id: self.id,
                name: self.name,
                description: self.description,
                r#type: self.r#type,
                connection_type: self.connection_type,
                arn: self.arn,
                owner_arn: self.owner_arn,
                lifecycle: self.lifecycle,
                managed_credentials_status: self.managed_credentials_status,
            }
        }
    }
}
impl Environment {
    /// Creates a new builder-style object to manufacture [`Environment`](crate::model::Environment)
    pub fn builder() -> crate::model::environment::Builder {
        crate::model::environment::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ManagedCredentialsStatus {
    DisabledByCollaborator,
    DisabledByDefault,
    DisabledByOwner,
    EnabledByOwner,
    EnabledOnCreate,
    FailedRemovalByCollaborator,
    FailedRemovalByOwner,
    PendingRemovalByCollaborator,
    PendingRemovalByOwner,
    PendingStartRemovalByCollaborator,
    PendingStartRemovalByOwner,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ManagedCredentialsStatus {
    fn from(s: &str) -> Self {
        match s {
            "DISABLED_BY_COLLABORATOR" => ManagedCredentialsStatus::DisabledByCollaborator,
            "DISABLED_BY_DEFAULT" => ManagedCredentialsStatus::DisabledByDefault,
            "DISABLED_BY_OWNER" => ManagedCredentialsStatus::DisabledByOwner,
            "ENABLED_BY_OWNER" => ManagedCredentialsStatus::EnabledByOwner,
            "ENABLED_ON_CREATE" => ManagedCredentialsStatus::EnabledOnCreate,
            "FAILED_REMOVAL_BY_COLLABORATOR" => {
                ManagedCredentialsStatus::FailedRemovalByCollaborator
            }
            "FAILED_REMOVAL_BY_OWNER" => ManagedCredentialsStatus::FailedRemovalByOwner,
            "PENDING_REMOVAL_BY_COLLABORATOR" => {
                ManagedCredentialsStatus::PendingRemovalByCollaborator
            }
            "PENDING_REMOVAL_BY_OWNER" => ManagedCredentialsStatus::PendingRemovalByOwner,
            "PENDING_START_REMOVAL_BY_COLLABORATOR" => {
                ManagedCredentialsStatus::PendingStartRemovalByCollaborator
            }
            "PENDING_START_REMOVAL_BY_OWNER" => {
                ManagedCredentialsStatus::PendingStartRemovalByOwner
            }
            other => ManagedCredentialsStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ManagedCredentialsStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ManagedCredentialsStatus::from(s))
    }
}
impl ManagedCredentialsStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ManagedCredentialsStatus::DisabledByCollaborator => "DISABLED_BY_COLLABORATOR",
            ManagedCredentialsStatus::DisabledByDefault => "DISABLED_BY_DEFAULT",
            ManagedCredentialsStatus::DisabledByOwner => "DISABLED_BY_OWNER",
            ManagedCredentialsStatus::EnabledByOwner => "ENABLED_BY_OWNER",
            ManagedCredentialsStatus::EnabledOnCreate => "ENABLED_ON_CREATE",
            ManagedCredentialsStatus::FailedRemovalByCollaborator => {
                "FAILED_REMOVAL_BY_COLLABORATOR"
            }
            ManagedCredentialsStatus::FailedRemovalByOwner => "FAILED_REMOVAL_BY_OWNER",
            ManagedCredentialsStatus::PendingRemovalByCollaborator => {
                "PENDING_REMOVAL_BY_COLLABORATOR"
            }
            ManagedCredentialsStatus::PendingRemovalByOwner => "PENDING_REMOVAL_BY_OWNER",
            ManagedCredentialsStatus::PendingStartRemovalByCollaborator => {
                "PENDING_START_REMOVAL_BY_COLLABORATOR"
            }
            ManagedCredentialsStatus::PendingStartRemovalByOwner => {
                "PENDING_START_REMOVAL_BY_OWNER"
            }
            ManagedCredentialsStatus::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "DISABLED_BY_COLLABORATOR",
            "DISABLED_BY_DEFAULT",
            "DISABLED_BY_OWNER",
            "ENABLED_BY_OWNER",
            "ENABLED_ON_CREATE",
            "FAILED_REMOVAL_BY_COLLABORATOR",
            "FAILED_REMOVAL_BY_OWNER",
            "PENDING_REMOVAL_BY_COLLABORATOR",
            "PENDING_REMOVAL_BY_OWNER",
            "PENDING_START_REMOVAL_BY_COLLABORATOR",
            "PENDING_START_REMOVAL_BY_OWNER",
        ]
    }
}
impl AsRef<str> for ManagedCredentialsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Information about the current creation or deletion lifecycle state of an Cloud9 development
/// environment.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct EnvironmentLifecycle {
    /// <p>The current creation or deletion lifecycle state of the environment.</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>CREATING</code>: The environment is in the process of being created.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>CREATED</code>: The environment was successfully created.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>CREATE_FAILED</code>: The environment failed to be created.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>DELETING</code>: The environment is in the process of being deleted.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>DELETE_FAILED</code>: The environment failed to delete.</p>
    /// </li>
    /// </ul>
    pub status: std::option::Option<crate::model::EnvironmentLifecycleStatus>,
    /// <p>Any informational message about the lifecycle state of the environment.</p>
    pub reason: std::option::Option<std::string::String>,
    /// <p>If the environment failed to delete, the Amazon Resource Name (ARN) of the related Amazon Web Services
    /// resource.</p>
    pub failure_resource: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for EnvironmentLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EnvironmentLifecycle");
        formatter.field("status", &self.status);
        formatter.field("reason", &self.reason);
        formatter.field("failure_resource", &self.failure_resource);
        formatter.finish()
    }
}
/// See [`EnvironmentLifecycle`](crate::model::EnvironmentLifecycle)
pub mod environment_lifecycle {
    /// A builder for [`EnvironmentLifecycle`](crate::model::EnvironmentLifecycle)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::EnvironmentLifecycleStatus>,
        pub(crate) reason: std::option::Option<std::string::String>,
        pub(crate) failure_resource: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The current creation or deletion lifecycle state of the environment.</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>CREATING</code>: The environment is in the process of being created.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>CREATED</code>: The environment was successfully created.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>CREATE_FAILED</code>: The environment failed to be created.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>DELETING</code>: The environment is in the process of being deleted.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>DELETE_FAILED</code>: The environment failed to delete.</p>
        /// </li>
        /// </ul>
        pub fn status(mut self, input: crate::model::EnvironmentLifecycleStatus) -> Self {
            self.status = Some(input);
            self
        }
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::EnvironmentLifecycleStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>Any informational message about the lifecycle state of the environment.</p>
        pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
            self.reason = Some(input.into());
            self
        }
        pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.reason = input;
            self
        }
        /// <p>If the environment failed to delete, the Amazon Resource Name (ARN) of the related Amazon Web Services
        /// resource.</p>
        pub fn failure_resource(mut self, input: impl Into<std::string::String>) -> Self {
            self.failure_resource = Some(input.into());
            self
        }
        pub fn set_failure_resource(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.failure_resource = input;
            self
        }
        /// Consumes the builder and constructs a [`EnvironmentLifecycle`](crate::model::EnvironmentLifecycle)
        pub fn build(self) -> crate::model::EnvironmentLifecycle {
            crate::model::EnvironmentLifecycle {
                status: self.status,
                reason: self.reason,
                failure_resource: self.failure_resource,
            }
        }
    }
}
impl EnvironmentLifecycle {
    /// Creates a new builder-style object to manufacture [`EnvironmentLifecycle`](crate::model::EnvironmentLifecycle)
    pub fn builder() -> crate::model::environment_lifecycle::Builder {
        crate::model::environment_lifecycle::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum EnvironmentLifecycleStatus {
    Created,
    CreateFailed,
    Creating,
    DeleteFailed,
    Deleting,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for EnvironmentLifecycleStatus {
    fn from(s: &str) -> Self {
        match s {
            "CREATED" => EnvironmentLifecycleStatus::Created,
            "CREATE_FAILED" => EnvironmentLifecycleStatus::CreateFailed,
            "CREATING" => EnvironmentLifecycleStatus::Creating,
            "DELETE_FAILED" => EnvironmentLifecycleStatus::DeleteFailed,
            "DELETING" => EnvironmentLifecycleStatus::Deleting,
            other => EnvironmentLifecycleStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for EnvironmentLifecycleStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(EnvironmentLifecycleStatus::from(s))
    }
}
impl EnvironmentLifecycleStatus {
    pub fn as_str(&self) -> &str {
        match self {
            EnvironmentLifecycleStatus::Created => "CREATED",
            EnvironmentLifecycleStatus::CreateFailed => "CREATE_FAILED",
            EnvironmentLifecycleStatus::Creating => "CREATING",
            EnvironmentLifecycleStatus::DeleteFailed => "DELETE_FAILED",
            EnvironmentLifecycleStatus::Deleting => "DELETING",
            EnvironmentLifecycleStatus::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "CREATED",
            "CREATE_FAILED",
            "CREATING",
            "DELETE_FAILED",
            "DELETING",
        ]
    }
}
impl AsRef<str> for EnvironmentLifecycleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum ConnectionType {
    ConnectSsh,
    ConnectSsm,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ConnectionType {
    fn from(s: &str) -> Self {
        match s {
            "CONNECT_SSH" => ConnectionType::ConnectSsh,
            "CONNECT_SSM" => ConnectionType::ConnectSsm,
            other => ConnectionType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ConnectionType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ConnectionType::from(s))
    }
}
impl ConnectionType {
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionType::ConnectSsh => "CONNECT_SSH",
            ConnectionType::ConnectSsm => "CONNECT_SSM",
            ConnectionType::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["CONNECT_SSH", "CONNECT_SSM"]
    }
}
impl AsRef<str> for ConnectionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum EnvironmentType {
    Ec2,
    Ssh,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for EnvironmentType {
    fn from(s: &str) -> Self {
        match s {
            "ec2" => EnvironmentType::Ec2,
            "ssh" => EnvironmentType::Ssh,
            other => EnvironmentType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for EnvironmentType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(EnvironmentType::from(s))
    }
}
impl EnvironmentType {
    pub fn as_str(&self) -> &str {
        match self {
            EnvironmentType::Ec2 => "ec2",
            EnvironmentType::Ssh => "ssh",
            EnvironmentType::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["ec2", "ssh"]
    }
}
impl AsRef<str> for EnvironmentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}