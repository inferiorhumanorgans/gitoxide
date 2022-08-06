use bstr::BStr;

/// The way to interpret a refspec.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Mode {
    /// Apply standard rules for refspecs which are including refs with specific rules related to allowing fast forwards of destinations.
    Normal,
    /// Even though according to normal rules a non-fastforward would be denied, override this and reset a ref forcefully in the destination.
    Force,
    /// Instead of considering matching refs included, we consider them excluded. This applies only to the source side of a refspec.
    Negative,
}

/// What operation to perform with the refspec.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Operation {
    /// The `src` side is local and the `dst` side is remote.
    Push,
    /// The `src` side is remote and the `dst` side is local.
    Fetch,
}

pub enum Instruction<'a> {
    Push(Push<'a>),
    Fetch(Fetch<'a>),
}

pub enum Push<'a> {
    /// Push a single ref knowing only one ref name.
    SingleMatching {
        /// The name of the ref to push from `src` to `dest`.
        src_and_dest: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
    /// Exclude a single ref.
    ExcludeSingle {
        /// A single full ref name to exclude.
        src: &'a BStr,
    },
    /// Exclude multiple refs with single `*` glob.
    ExcludeMultipleWithGlob {
        /// A ref pattern with a single `*`.
        src: &'a BStr,
    },
    /// Push a single ref or refspec to a known destination ref.
    Single {
        /// The source ref or refspec to push.
        src: &'a BStr,
        /// The ref to update with the object from `src`.
        dest: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
    /// Push a multiple refs to matching destination refs, with exactly a single glob on both sides.
    MultipleWithGlob {
        /// The source ref to match against all refs for pushing.
        src: &'a BStr,
        /// The ref to update with object obtained from `src`, filling in the `*` with the portion that matched in `src`.
        dest: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
}

pub enum Fetch<'a> {
    Only {
        /// The ref name to fetch on the remote side, without updating the local side.
        src: &'a BStr,
    },
    /// Exclude a single ref.
    ExcludeSingle {
        /// A single full ref name to exclude.
        src: &'a BStr,
    },
    /// Exclude multiple refs with single `*` glob.
    ExcludeMultipleWithGlob {
        /// A ref pattern with a single `*`.
        src: &'a BStr,
    },
    AndUpdateSingle {
        /// The ref name to fetch on the remote side.
        src: &'a BStr,
        /// The local destination to update with what was fetched.
        dest: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
    /// Similar to `FetchAndUpdate`, but src and destination contain a single glob to fetch and update multiple refs.
    AndUpdateMultipleWithGlob {
        /// The ref glob to match against all refs on the remote side for fetching.
        src: &'a BStr,
        /// The local destination to update with what was fetched by replacing the single `*` with the matching portion from `src`.
        dest: &'a BStr,
        /// If true, allow non-fast-forward updates of `dest`.
        allow_non_fast_forward: bool,
    },
}
