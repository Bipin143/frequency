# Delegations
This document describes the permissioned delegation of actions, largely, but not limited to, account creation and announcing messages by the owner of an `MsaId` on chain on behalf of the owner of another `MsaId`.

## Table of Contents
* [Context and Scope](https://github.com/LibertyDSNP/meta#installation)
* [Problem Statement](https://github.com/LibertyDSNP/meta#dependenciesrequirements)
* [Goals and Non-Goals](https://github.com/LibertyDSNP/meta#configuration)
* [Proposal](https://github.com/LibertyDSNP/meta#examples)
* [Benefits and Risks](https://github.com/LibertyDSNP/meta#roadmap)
* [Alternatives and Rationale](https://github.com/LibertyDSNP/meta#support)
* [Additional Resources](https://github.com/LibertyDSNP/meta#contributing)
* [Glossary](https://github.com/LibertyDSNP/meta#overview)


## Context and Scope
This document describes how a delegation is created and validated on chain, and outlines an API.
Delegation to one account can be used to perform tasks on behalf of another account.
Some examples of delegated actions and delegated permissions are given.
It's expected that the actions and permissions that are implemented for delegation will evolve as needed.

## Problem Statement
The primary motivation for delegation is to support End Users of the DSNP platform, however, it is expected that delegation will be used in other ways.

Market research makes it clear that End Users are extremely reluctant to pay to use applications, particularly social networks.
This means there needs to be some way to onboard End Users and relay their activity through the DSNP platform without charging them.
On Ethereum and now on MRC, the use of authorized Delegates enables the creation of End User accounts as well as processing and storing user messages and other data for the End Users, paid for by a Provider, who can recoup these costs by other means (outside the scope of this Design Document).
The vast majority of this activity will not reside on chain, however, MRC needs to be able to coordinate the exchange of data, and to securely allow an End User or any other type of account holder to manage their Delegates.
The delegation is managed by assigning each account, called a Message Sourcing Account or MSA, an ID number, called an MsaId.

## Goals and Non-Goals
Delegation, roughly speaking, must allow all Create, Read, Update and Delete (CRUD) operations by a Delegating MSA to fulfill the purpose of giving other MSAs proper authority over their Delegates.
Put another way, delegation must have the following properties:
* **Authorizable** - delegations can be authorized with specific permissions by MSAs.
* **Verifiable** - there is a way to check that Providers are doing things only when authorized and only what they are authorized to do.
* **Transparent** - delegations can be readable by anyone, in order to maximize opportunities to police Provider actions.
* **Changeable** - a Delegator can change Delegate permissions to give MSAs control over what tasks are permitted to the Delegate.
* **Revocable** - a Delegator can withdraw permissions completely from the Delegate.

### Non-Goals
* Doesn't cover handling the retirement of an `MsaId`, which is a possible future feature and would affect delegation validation and queries.
* Delegated removal would allow removing any other delegate without substituting itself as the new delegate. Such an endpoint presents serious enough issues that it should be discussed and designed separately, if it's to be implemented at all.
* Does not specify what the permissions are nor the permissions data type.
* Does not specify a value for pallet constants, only when there should be one. These values should be determined by such factors as storage costs and performance.
* Does not include a "block/ban" feature for delegation, which is under discussion; the belief is that a Delegate also ought to be able to permanently refuse service to a given `MsaId`, which further supports the idea of a mutually agreed upon relationship.

## Proposal
The proposed solution is to give End Users the ability to create an on-chain `MsaId` through an authorized delegate. End Users can also transparently authorize and manage their own Delegates and permissions, either directly using a native token or through an explicitly authorized Delegate. Additionally, we allow `MsaIds` to be directly purchased using a native token.

### API (extrinsics)
* All names are placeholders and may be changed.
* All extrinsics must emit an appropriate event with all parameters for the call, unless otherwise specified.
* Errors in the extrinsics must have different, reasonably-named error enums for each type of error for ease of debugging.
* "Owner only" means the caller must own the delegator`MsaId`.
* Events are not deposited for read-only extrinsic calls.

#### create_account_with_delegate
Creates a new `MsaId` on behalf of an MSA and adds the origin as the MSA's delegate.
  * Parameters:
      1. `payload`:
         1. `data` - this is what the MSA owner must sign and provide to the delegate beforehand.
             * `msa_id` - the delegate, of type `MsaId`
             * `permission` a value indicating the permission to be given to the delegate
         2. `public_key` - The authorizing key used to create `signature`
         3. `signature` - The signature of the hash of `data`
    * Event:  `IdentityCreated`, with the `public_key`, the newly created `MsaId`, and `msa_id`
    * Restrictions:  origin must own `msa_id` in the payload.

#### add_self_as_delegate(payload)
Adds the `MsaId` in the payload as a delegate, to an MSA owning `delegator_msa_id`
  * Parameters:
      1. `payload`: authorization data signed by the delegating account
          1. `data` - this is what the MSA owner must sign and provide to the delegate beforehand.
              * `delegate_msa_id` - the delegate's `MsaId`, i.e. the caller's `MsaId`
              * `permission` a value indicating the permission to be given to the delegate
          2. `public_key` - The authorizing key used to create `signature`
          3. `signature` - The signature of the hash of `data`
    * Event: `DelegateAddedSelf` with `public_key`, `msa_id`, and `delegate_msa_id`
    * Restrictions:  Caller/origin must own `delegate_msa_id`. The `public_key` MSA must own `msa_id`.

#### replace_delegate_with_self(payload)
By signed authorization of owner of `delegator`, the `delegator`-->`old_delegate` relationship is removed and replaced with a`delegator`-->`new_delegate` relationship. The purpose of this extrinsic is to allow an End User to change delegates, for example, if they want to switch to a different DSNP dApp.
* **Parameters:**
  1. `payload`: authorization data signed by the delegating account
     * `data` - what the MSA owner must sign and provide to the delegate beforehand.
       * `new_delegate` - the new delegate's `MsaId`, i.e. the caller's `MsaId`
       * `old_delegate` - the `MsaId` of the delegate to be replaced
       * `msa_id` - the `MsaId` of the authorizing MSA
       * `permission` a value indicating the permission to be given to the *new* delegate
  2. `public_key` - The authorizing key used to create `signature`
  3. `signature` - The signature of the hash of `data`
* **Event**: `DelegateReplacedWithSelf` with the `public_key`, `msa_id`,`old_delegate` and `new_delegat
* **Restrictions**:
  * Caller/origin must own `new_delegate`.
  * `public_key` MSA must own `msa_id`.
  * `old_delegate` must be a delegate of `msa_id`.

#### update_delegate_self(payload)
By signed authorization of owner of `delegator`, `delegate`'s own permissions are updated to `new_permissions`.
* Parameters
  1. `payload`: authorization data signed by the delegating account
     1. `data` - what the MSA owner must sign and provide to the delegate beforehand.
               * `delegate` - the delegate's `MsaId`, i.e. the caller's `MsaId`
               * `msa_id` - the `MsaId` of the authorizing MSA.
               * `new_permission` a value indicating the new permission to be given to this delegate
     2. `public_key` - The authorizing key used to create `signature`
     3. `signature` - The signature of the hash of `data`
* Event: `DelegateUpdatedSelf` with `delegate`, `msa_id`, `new_permissions`

#### remove_delegate_self(payload)
Delegate removes themselves from the specified `msa_id` in the payload.  This function allows a delegate to control access to its services, for example, in the case of an End User that violates Terms of Service.

#### create_msa_id()
Directly creates a `MsaId` for the origin (caller) MSA with no delegates.
This is a signed call directly from the caller, so the owner of the new `MsaId` pays the fees for `MsaId` creation.

* Event: `IdentityCreated`, with the caller's Public Key, the newly created `MsaId`, and an empty delegate `MsaId`.

#### update_delegate(delegator, delegate, permissions)
Changes the permissions for an existing `delegator`-->`delegate` relationship.
This is a signed call directly from the delegator's MSA.
The *delegator* account pays the fees.
  * Parameters: the same as `add_delegate`.
  * Restrictions:  **Owner only**.
  * Event: `DelegateUpdated` with `delegator`, `delegate`, `new_permissions`

#### remove_delegate(delegator,delegate)
Deletes a delegate's entry from the list of delegates for the provided `MsaId`.
This is a signed call directly from the delegator's MSA.
The *delegator* account pays the fees, if any.
* Parameters:
       1. `delegator` - the `MsaId` removing the delegate
       2. `delegate` - the `MsaId` of the delegate to be removed
    * Restrictions:  **Owner only**.
    * Event: `DelegateRemoved` with `delegator`, `delegate`

#### remove_msa_id(msa_id)
Deletes the `MsaId` from the registry entirely.
* Restrictions:  Owner [and/or sudoer?]
* Event: `MsaIdRemoved` with `msa_id`

### Custom RPC endpoints
#### get_account_ids(msa_id)
Retrieve a list of public keys of up to `MaxKeys` size for the provided `MsaId`, or `None()` if the `MsaId` does not exist.

#### get_msa_id(public_key)
Retrieve the `MsaId` for the provided `public_key`, or `None()` if`public_key` does not exist.

#### validate_delegate(delegator, delegate, permission)
Verify that the provided delegate `MsaId` is a delegate of the delegator, and has the given permission value.
Returns `Ok(true)` if delegate is valid, `Ok(false)` if not.
Throws an Error enum indicating if either delegate or delegator does not exist.

* Parameters:
    1. `delegator`: the `MsaId` of the delegator
    2. `delegate`: the `MsaId` of the delegate to verify.
    3. `permission`: the `Permission` value to check against what is stored for these delegates.
        If this is the `Zero()` value, it checks only that this is a delegate for this delegator `MsaId`.

#### validate_delegate_for_ids(delegate_msa_id, msa_ids, permission)
Validate a delegate for a bunch of `MsaIds` against the provided `permission`.
This call is intended for validating messages in a batch, so this function would be an all-or-nothing check.
If the permission stored for a given `MsaId` exceeds the parameter, the check for that `MsaId` passes.
For example, if a delegate has *all* permissions set, then querying for a subset of permissions will pass.

* Returns: `Ok(true)` if delegate is valid for all ids *and* the delegate exists *and* all of the ids exist, `Ok(false)` otherwise.
  It's up to the caller to decide if they want to figure out why validation failed and how.
* Parameters:
  1. `delegate_msa_id`: the `MsaId` of the delegate to verify.
  2. `msa_ids`: the list of `MsaIds` to check. The list of `MsaIds` should have a sensible maximum length.
  3. `permission`: the `Permission` value to check against. Since the call is intended for validating a batch, `permission` is a single value.

### Related Pallet configuration, constants and enumerations
* `MaxDelegates` - maximum number of delegates a given ID is allowed to have. Pallet configuration.
* `Permissions` type should be explicitly set in `types.rs` for the pallet
* `Permissions` definitions to be discussed.  Possibilities:  Owner, Announcer, Expiring

### Storage
* Delegations are stored as a Double-key map of Delegator `MsaId` --> Delegate `MsaId`. The data stored contains the `Permission` for that relationship:
```rust
pub(super) type Delegates<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MsaId,
		Blake2_128Concat,
		MsaId,
		Permission,
		OptionQuery,
	>;
```

## Benefits and Risks
As stated earlier, one of the primary intended benefits of delegation is to allow feeless account creation and announcing.

There is a risk of abuse with delegation of announcements, since this makes it possible for a delegate to, for example, modify the End User's messages before batching them. The announcer would have to be caught and the End User must react after the fact, instead of the announcer being technologically prevented from this type of dishonesty.

There is another risk of abuse for any other type of delegated call if the wallet that provides the signing capability does not make it very clear to the End User what they're signing.

## Alternatives and Rationale
### End User pays for existential deposit
We briefly discussed the possibility of requiring a small token deposit to create their account. We decided against this option because:
1. As mentioned above, people don't expect and won't pay to use social media.
2. Onboarding would be a problem; even if they did want to pay even a small amount, getting people access to a token is tremendously difficult at this time, requiring unacceptable tradeoffs.
3. We would be unable to serve people who are unbanked or don't have access to crypto trading platforms.

### dApp Developer pays for existential deposit
One alternative to allow for account creation at no cost to the End User was the dApp developer MSA sends an existential deposit to the account to create it.
We decided against this option for a number of reasons.
1. It could create a potential for abuse and token loss by those creating numerous fake accounts and then removing the dApp Public Key as a delegate.
2. We have the ability not to require an existential deposit, and felt this to be a better option in this particular case.

### End user pays to announce, with no possibility of delegating
An alternative for delegating announcement capabilities was to have each End User pay for their own announcing.
This was ruled out as the sole solution because:
1. The average person can't or won't pay to use social media.
2. Making End Users pay to announce would require people to sign transactions every time they make any updates — all posts, all reactions, all replies, all profile changes, all follows/unfollows, etc. Having to do this would be too annoying for the End User.

This design still include some direct pay endpoints, so even if an End User did not want to trust a delegate, they could still pay for all of their announcing if they want to assume the cost of running a node and pay directly.

### Permissioned delegation is an industry standard
Furthermore, permissioned delegation via verifiable strong cryptographic signature is a well-known and tested feature in smart contracts of distributed blockchain-based applications.

### Deferred features
#### An "effective block range" for delegates
Including an effective block range in the delegate storage data would allow delegates to be expired, not just removed.  A block range could better support features like Tombstone, blocking, and retiring an `MsaId`.  Effective block range is deferred because those features have not been fully defined.

#### add_delegate(delegator, delegate, permissions)
Directly adding a delegate, with or without a delegate's permission, is not to be implemented at this time. The original use case was for a potential wallet app to support browsing and adding delegates. Adding/replacing a delegate for an existing account with an `MsaId` could still be done using the delegated methods, `add_self_as_delegate` or `replace_delegate_with_self`.  A direct add brought up concerns about potential risks of adding a delegate without the delegate's knowledge. For example, if the delegate has removed the delegator for legitimate reasons, such as if the End User violated the delegate's Terms of Service, then the delegate ought to be able to prevent them from adding the delegate again just by paying for it.

## Glossary
* **Delegate**: An `MsaId` that has been granted specific permissions by its Delegator.
* **Delegator**: An `MsaId` that has granted specific permissions to a Delegate.
* **MSA**: Message Sourcing Account. A collection of key pairs which can have a specific token balance.
* **Public Key**: A 32-byte (u256) number that is used to refer to an on-chain MSA and verify signatures. It is one of the keys of an MSA key pair
* **MsaId**: An 8-byte (u64) number used as a lookup and storage key for delegations, among other things
* **Provider**: A company or individual operating an on-chain Delegate MSA in order to post MRC transactions on behalf of other MSAs.  Provider MSAs will have one or more token balances.
* **End User**: Groups or individuals that own an MSA that is not a Provider MSA.
