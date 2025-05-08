// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct AddUserParams<
    T1: crate::BytesSql,
    T2: crate::BytesSql,
    T3: crate::BytesSql,
    T4: crate::ArraySql<Item = T3>,
> {
    pub ehash: T1,
    pub primary_public_key: T2,
    pub additional_public_keys: Option<T4>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: uuid::Uuid,
    pub ehash: Vec<u8>,
    pub primary_public_key: Vec<u8>,
    pub additional_public_keys: Option<Vec<Vec<u8>>>,
    pub created_at: crate::types::time::TimestampTz,
}
pub struct UserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub ehash: &'a [u8],
    pub primary_public_key: &'a [u8],
    pub additional_public_keys: Option<crate::ArrayIterator<'a, &'a [u8]>>,
    pub created_at: crate::types::time::TimestampTz,
}
impl<'a> From<UserBorrowed<'a>> for User {
    fn from(
        UserBorrowed {
            id,
            ehash,
            primary_public_key,
            additional_public_keys,
            created_at,
        }: UserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            ehash: ehash.into(),
            primary_public_key: primary_public_key.into(),
            additional_public_keys: additional_public_keys.map(|v| v.map(|v| v.into()).collect()),
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct UserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<UserBorrowed, tokio_postgres::Error>,
    mapper: fn(UserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UserBorrowed) -> R) -> UserQuery<'c, 'a, 's, C, R, N> {
        UserQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
/// Creates a new user with the given encryption hash and public keys
/// Parameters:
/// ehash: 32-byte hash
/// primary_public_key: 32-byte primary public key
/// additional_public_keys: Optional array of additional 32-bit public keys
pub fn add_user() -> AddUserStmt {
    AddUserStmt(crate::client::async_::Stmt::new(
        "INSERT INTO \"user\" ( ehash, primary_public_key, additional_public_keys ) VALUES ( $1, $2, COALESCE($3, array[]::BYTEA[]) ) RETURNING id, ehash, primary_public_key, additional_public_keys, created_at",
    ))
}
pub struct AddUserStmt(crate::client::async_::Stmt);
impl AddUserStmt {
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::BytesSql,
        T2: crate::BytesSql,
        T3: crate::BytesSql,
        T4: crate::ArraySql<Item = T3>,
    >(
        &'s mut self,
        client: &'c C,
        ehash: &'a T1,
        primary_public_key: &'a T2,
        additional_public_keys: &'a Option<T4>,
    ) -> UserQuery<'c, 'a, 's, C, User, 3> {
        UserQuery {
            client,
            params: [ehash, primary_public_key, additional_public_keys],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<UserBorrowed, tokio_postgres::Error> {
                Ok(UserBorrowed {
                    id: row.try_get(0)?,
                    ehash: row.try_get(1)?,
                    primary_public_key: row.try_get(2)?,
                    additional_public_keys: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| User::from(it),
        }
    }
}
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::BytesSql,
        T2: crate::BytesSql,
        T3: crate::BytesSql,
        T4: crate::ArraySql<Item = T3>,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        AddUserParams<T1, T2, T3, T4>,
        UserQuery<'c, 'a, 's, C, User, 3>,
        C,
    > for AddUserStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a AddUserParams<T1, T2, T3, T4>,
    ) -> UserQuery<'c, 'a, 's, C, User, 3> {
        self.bind(
            client,
            &params.ehash,
            &params.primary_public_key,
            &params.additional_public_keys,
        )
    }
}
/// Retrieves a user by their UUID
/// Parameters:
/// id: The UUID of the user to retrieve
pub fn get_user() -> GetUserStmt {
    GetUserStmt(crate::client::async_::Stmt::new(
        "SELECT id, ehash, primary_public_key, additional_public_keys, created_at FROM \"user\" WHERE id = $1",
    ))
}
pub struct GetUserStmt(crate::client::async_::Stmt);
impl GetUserStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        id: &'a uuid::Uuid,
    ) -> UserQuery<'c, 'a, 's, C, User, 1> {
        UserQuery {
            client,
            params: [id],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<UserBorrowed, tokio_postgres::Error> {
                Ok(UserBorrowed {
                    id: row.try_get(0)?,
                    ehash: row.try_get(1)?,
                    primary_public_key: row.try_get(2)?,
                    additional_public_keys: row.try_get(3)?,
                    created_at: row.try_get(4)?,
                })
            },
            mapper: |it| User::from(it),
        }
    }
}
