//! 
//! sample
//!
//! A crate for simplifying generic audio sample processing. Use the Sample
//! trait to remain generic across any audio bit-depth.
//!

/// Represents a sample from a Wave between -1.0 and 1.0.
pub type Wave = f32;
/// Represents a Wave amplitude between 0.0 and 1.0.
pub type Amplitude = f32;

/// A dynamic representation of common Sample Formats.
pub enum Format {
    /// 32-bit floating point.
    F32,
    /// 32-bit integer.
    I32,
    /// 16-bit integer.
    I16,
    /// 8-bit integer.
    I8,
    /// 32-bit unsigned integer.
    U32,
    /// 16-bit unsigned integer. 0 corresponds to half of ::std::u16::MAX.
    U16,
    /// 8-bit unsigned integer.
    U8,
}

impl Format {
    /// Return the size in bytes for the Format.
    pub fn size_in_bytes(&self) -> usize {
        use std::mem::size_of;
        match *self {
            Format::F32 => size_of::<f32>(),
            Format::I32 => size_of::<i32>(),
            Format::I16 => size_of::<i16>(),
            Format::I8  => size_of::<i8>(),
            Format::U32 => size_of::<u32>(),
            Format::U16 => size_of::<u16>(),
            Format::U8  => size_of::<u8>(),
        }
    }
}

/// A trait for working generically across different sample types.
pub trait Sample:
    Copy +
    Clone +
    ::std::default::Default +
    ::std::fmt::Debug +
    PartialOrd +
    PartialEq +
    ::std::ops::Add<Output=Self> +
    ::std::ops::Sub<Output=Self> +
    ::std::ops::Mul<Output=Self> +
    ::std::ops::Div<Output=Self> +
    ::std::ops::Rem<Output=Self> +
{

    /// Construct a sample from a wave sample between -1. and 1.
    fn from_wave(Wave) -> Self;

    /// Convert to a wave sample between -1. and 1.
    fn to_wave(self) -> Wave;

    /// Return the sample format as a method.
    fn sample_format(&self) -> Format;

    /// Return the format of the sample.
    fn format<S: Sample>() -> Format {
        let sample: S = ::std::default::Default::default();
        sample.sample_format()
    }

    /// Multiply by a given amplitude.
    #[inline]
    fn mul_amp(self, amp: f32) -> Self {
        Sample::from_wave(self.to_wave() * amp)
    }

    /// Construct a sample from an arbitrary Sample type.
    #[inline]
    fn from_sample<S: Sample>(sample: S) -> Self {
        Sample::from_wave(sample.to_wave())
    }

    /// Construct an arbitrary sample type from a sample of this Self type.
    #[inline]
    fn to_sample<S: Sample>(self) -> S {
        Sample::from_wave(self.to_wave())
    }

    /// A silent sample.
    #[inline]
    fn zero() -> Self { ::std::default::Default::default() }

    /// Sum the working buffer onto the output buffer after multiplying it by volume per channel.
    #[inline]
    fn add_buffers(output: &mut [Self], working: &[Self], vol_per_channel: &[Amplitude]) {
        let channels = vol_per_channel.len();
        let output_frames = output.chunks_mut(channels);
        let working_frames = working.chunks(channels);
        for (output_frame, working_frame) in output_frames.zip(working_frames) {
            let output_channels = output_frame.iter_mut();
            let working_channels = working_frame.iter();
            let channel_vols = vol_per_channel.iter();
            for ((o, w), vol) in output_channels.zip(working_channels).zip(channel_vols) {
                *o = *o + w.mul_amp(*vol);
            }
        }
    }

    /// Zero a given buffer of samples.
    #[inline]
    fn zero_buffer(buffer: &mut [Self]) {
        for sample in buffer.iter_mut() { *sample = Sample::zero() }
    }

}

// FLOATING POINT NUMBERS.

impl Sample for f32 {
    #[inline]
    fn from_wave(wave: Wave) -> f32 { wave }
    #[inline]
    fn to_wave(self) -> Wave { self }
    fn sample_format(&self) -> Format { Format::F32 }
}

// SIGNED INTEGERS.

/// Slight headroom is needed between the max value for 32-bit integers due
/// to resolution error when converting to and from 32-bit floating points.
const RESOLUTION_HEADROOM_I32: i32 = 128;

impl Sample for i32 {
    #[inline]
    fn from_wave(wave: Wave) -> i32 {
        const MAX: Wave = (::std::i32::MAX - RESOLUTION_HEADROOM_I32) as Wave;
        (MAX * wave) as i32
    }
    #[inline]
    fn to_wave(self) -> Wave {
        const MAX: Wave = (::std::i32::MAX - RESOLUTION_HEADROOM_I32) as Wave;
        self as Wave / MAX
    }
    fn sample_format(&self) -> Format { Format::I32 }
}

impl Sample for i16 {
    #[inline]
    fn from_wave(wave: Wave) -> i16 {
        const MAX: Wave = ::std::i16::MAX as Wave;
        (MAX * wave) as i16
    }
    #[inline]
    fn to_wave(self) -> Wave {
        const MAX: Wave = ::std::i16::MAX as Wave;
        self as Wave / MAX
    }
    fn sample_format(&self) -> Format { Format::I16 }
}

impl Sample for i8 {
    #[inline]
    fn from_wave(wave: Wave) -> i8 {
        const MAX: Wave = ::std::i8::MAX as Wave;
        (MAX * wave) as i8
    }
    #[inline]
    fn to_wave(self) -> Wave {
        const MAX: Wave = ::std::i8::MAX as Wave;
        self as Wave / MAX
    }
    fn sample_format(&self) -> Format { Format::I8 }
}

// UNSIGNED INTEGERS.

/// Slight headroom is needed between the max value for 32-bit unsigned integers due
/// to resolution error when converting to and from 32-bit floating points.
const RESOLUTION_HEADROOM_U32: u32 = 128;

impl Sample for u32 {
    #[inline]
    fn from_wave(wave: Wave) -> u32 {
        const HALF_MAX: Wave = ((::std::u32::MAX - RESOLUTION_HEADROOM_U32) / 2) as Wave;
        (HALF_MAX + HALF_MAX * wave) as u32
    }
    #[inline]
    fn to_wave(self) -> Wave {
        const MAX: Wave = (::std::u32::MAX - RESOLUTION_HEADROOM_U32) as Wave;
        (self as Wave / MAX) * 2.0 - 1.0
    }
    fn sample_format(&self) -> Format { Format::U32 }
}

impl Sample for u16 {
    #[inline]
    fn from_wave(wave: Wave) -> u16 {
        const HALF_MAX: Wave = (::std::u16::MAX / 2) as Wave;
        (HALF_MAX + HALF_MAX * wave) as u16
    }
    #[inline]
    fn to_wave(self) -> Wave {
        const MAX: Wave = ::std::u16::MAX as Wave;
        (self as Wave / MAX) * 2.0 - 1.0
    }
    fn sample_format(&self) -> Format { Format::U16 }
}

impl Sample for u8 {
    #[inline]
    fn from_wave(wave: Wave) -> u8 {
        const HALF_MAX: Wave = (::std::u8::MAX / 2) as Wave;
        (HALF_MAX + HALF_MAX * wave) as u8
    }
    #[inline]
    fn to_wave(self) -> Wave {
        const MAX: Wave = ::std::u8::MAX as Wave;
        (self as Wave / MAX) * 2.0 - 1.0
    }
    fn sample_format(&self) -> Format { Format::U8 }
}

