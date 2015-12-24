use std::ffi::NulError;
use std::ffi::CString;

use cassandra::statement::Statement;

use cassandra_sys::CassError;
use cassandra_sys::CassConsistency;
use cassandra_sys::cass_batch_set_consistency;
use cassandra_sys::cass_batch_add_statement;
use cassandra_sys::cass_batch_set_custom_payload;
use cassandra_sys::cass_batch_set_retry_policy;
use cassandra_sys::cass_batch_set_serial_consistency;
use cassandra_sys::cass_batch_set_timestamp;
use cassandra_sys::cass_batch_free;
use cassandra_sys::cass_batch_new;
use cassandra_sys::CASS_BATCH_TYPE_LOGGED;
use cassandra_sys::CASS_BATCH_TYPE_UNLOGGED;
use cassandra_sys::CASS_BATCH_TYPE_COUNTER;
use cassandra_sys::CassCustomPayload as _CassCustomPayload;
use cassandra::policy::retry::RetryPolicy;
use cassandra::consistency::Consistency;
use cassandra_sys::cass_custom_payload_free;
use cassandra_sys::cass_custom_payload_new;
use cassandra_sys::cass_custom_payload_set;

pub use cassandra_sys::CassBatch as _Batch;

pub struct Batch(pub *mut _Batch);

// FIXME find this a better home
pub struct CustomPayload(pub *mut _CassCustomPayload);

impl CustomPayload {
    pub fn new() -> Self {
        unsafe { CustomPayload(cass_custom_payload_new()) }
    }
    pub fn set(&self, name: String, value: &[u8]) -> Result<(), NulError> {
        unsafe {
            Ok(cass_custom_payload_set(self.0,
                                       try!(CString::new(name)).as_ptr(),
                                       value.as_ptr(),
                                       value.len() as u64))
        }
    }
}

impl Drop for CustomPayload {
    fn drop(&mut self) {
        unsafe { cass_custom_payload_free(self.0) }
    }
}

pub enum BatchType {
    LOGGED = CASS_BATCH_TYPE_LOGGED as isize,
    UNLOGGED = CASS_BATCH_TYPE_UNLOGGED as isize,
    COUNTER = CASS_BATCH_TYPE_COUNTER as isize,
}

impl Drop for Batch {
    ///Frees a batch instance. Batches can be immediately freed after being
    ///executed.
    fn drop(&mut self) {
        unsafe { cass_batch_free(self.0) }
    }
}

impl Batch {
    ///Creates a new batch statement with batch type.
    pub fn new(batch_type: BatchType) -> Batch {
        unsafe { Batch(cass_batch_new(batch_type as u32)) }
    }

    ///Sets the batch's consistency level
    pub fn set_consistency(&mut self, consistency: CassConsistency) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_consistency(self.0, consistency) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Sets the batch's serial consistency level.
    ///
    ///<b>Default:</b> Not set
    pub fn set_serial_consistency(&mut self, consistency: Consistency) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_serial_consistency(self.0, consistency.0) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    /// Sets the batch's timestamp.
    pub fn set_timestamp(&mut self, timestamp: i64) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_timestamp(self.0, timestamp) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Sets the batch's retry policy.
    pub fn set_retry_policy(&mut self, retry_policy: RetryPolicy) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_retry_policy(self.0, retry_policy.0) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Sets the batch's custom payload.
    pub fn set_custom_payload(&mut self,
                              custom_payload: CustomPayload)
                              -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_set_custom_payload(self.0, custom_payload.0) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }

    ///Adds a statement to a batch.
    pub fn add_statement(&mut self, statement: Statement) -> Result<&Self, CassError> {
        unsafe {
            match cass_batch_add_statement(self.0, statement.0) {
                0 => Ok(self),
                err => Err(err),
            }
        }
    }
}