#[cfg(feature = "futures-01")]
mod futures_01;
<<<<<<< HEAD

#[cfg(feature = "futures_preview")]
mod futures_preview;
#[cfg(feature = "futures_preview")]
pub use self::futures_preview::*;
||||||| 386969ba
#[cfg(feature = "futures-01")]
pub use self::futures_01::*;

#[cfg(feature = "futures_preview")]
mod futures_preview;
#[cfg(feature = "futures_preview")]
pub use self::futures_preview::*;
=======
>>>>>>> origin/master

#[cfg(feature = "futures-03")]
mod futures_03;
#[allow(unreachable_pub, unused_imports)]
#[cfg(feature = "futures-03")]
pub use futures_03::*;
