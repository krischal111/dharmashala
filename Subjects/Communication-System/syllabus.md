# Communication System
## Syllabus

Lectures: 4 Teaching Hours per week

Tutorial: 1 Teaching Hour per week

Practical: 3 Teaching Hours per week

## Course Objectives

To introduce the students to the principles and building blocks of nalog and digital communication systems.

## Theory

1. Introduction (2 hours)
    1. Analog and Digital communication sources, transmitters, transmission channels and receivers.
    2. Noise, distortion and interference. Fundamental limitation due to noise, distortion and interference.
    3. Types and reasons for modulation.

2. Representation of signals and sytems in communication (4 hours)
    1. Review of signals (types, mathematical representation and applications).
    2. Linear/non-linear, time variant/invarient systems. Impulse response and transfer function of a system. Properties of LTI systems.
    3. Low pass and band pass signals and systems, bandwidth of the system, distorionless transmission, the Hilbert transform and its properties.
    4. Complex envelops rectangular (in-phase and quadrature components) and polar representatoin of band pass band limited signals.

3. Spectral analysis (2 hours)
    1. Review of Fourier series and transform, energy and power, Parseval's theorem.
    2. Energy Density Spectrum, periodogram, power spectral density function (psdf).
    3. Power spectral density functions of harmonic signal and white noise.
    4. The autocorrelation (AC) function, relationshp between psdf and AC function

4. Amplitude Modulation and Demodulation (8 hours)
    1. Time domain expressions, frequency domain representation, modulation index, signal bandwidth of Amplitude Modulated (AM) signal.
    2. AM for a single tone message, carrier and side-band components, powers in carrier and side-band components, bandwidth and power efficiency.
    3. Generation of Double Side Band Full Carrier (DSB-FC) AM.
    4. Double Side Band Supressed Carrier AM (DSB-AM), time and frequency domain expressions, powers in side-bands, bandwidth and power efficiency.
    5. Generation of DSB-AM (balanced, ring modulators)
    6. Single Side Band Modulation, time and frequency domain expressions, powers.
    7. Generation of SSB (SSB filters and indirect method).
    8. Introduction of Vestigial Side Bands (VSB), Independent Side Bands (ISB) and Quadrature Amplitude Modulation.
    9. Demodulation of SSB using carrier reinsertion, carrier recovery circuits.
    10. Square law and envelop detection of DSB-FC.
    11. Demodulation of SSB using carrier reinsertion, carrier recovery circuits.
    12. Phase Locked Loop (PLL), basic concept, definitions, equations and applications, demodulation of AM using PLL.

5. Angle Modulaton and Demodulation (6 hours)
    1. Basic definitions, time domain expressions for Frequency Modulation (FM) and Phase Modulation (FM).
    2. Time domain expression for single tone modulated FM signals, spectral representation, Bessel's function and poperties.
    3. Bandwidth of FM, Carson's rule, narrow and wideband FM.
    4. Generation of FM (direct and Armstrong's methods).
    5. Demodulation of FM and PM signals, synchronous (PLL) and non-synchronous (limiter-discriminator) demodulation.
    6. Stereo FM, spectral details, encoder and decoder.
    7. Pre-emphasis and de-emphasis networks.
    8. The super-heterodyne radio receivers for AM and FM.

6. Source Coding and Sampling Theory (4 hours)
    1. Digital communication sources, transmitters, transmission channels and receivers.
    2. Source coding, coding efficiency, Shannon-Fano and Huffman codes, coding of continuous time signals (A/D conversion).
    3. Nyquist-Kotelnikov sampling theorem for strictly band-limited continuous time signal, time domain and frequency domain analysis, spectrum of sampled signal, reconstruction of sampled signal.
    4. Ideal, flat-top and natural sampling processes, sampling of band-pass signals, sub-sampling theory.
    5. Practical considerations: non-ideal sampling pulses (aperture effect), non-ideal reconstruction filter and time-limitness of the signal to be sampled (aliasing effects).

7. Pulse Modulation System (6 hours)
    1. Pulse amplitude Modulation (PAM), generation, bandwidth requirements, spectrum, reconstruction methods.
    2. Pulse position and pulse width modulations, generation, bandwidth requirements.
    3. Pulse code modulation as the result of analog to digital vocnversion, uniform quantization.
    4. Quantization noise, signal to quantization noise ratio in uniform quantization.
    5. Non uniform quantization, improvement in average SQNR for signals with high crest factor, comapnding techniques (µ and A law companding).
    6. Data rate and bandwidth of a PCM signal.
    7. Differntial PCM, encoder, decoder.
    8. Delta Modulation, encoder, decoder, noises in DM, SQNR. Comparison between PCM and DM.
    9. Parametric speech coding, vocoders.

8. Baseband Data Communication (6 hours)
    1. Introduction to information theory, measure of information, entropy, symbol rates and data (bit) rates.
    2. Shannon Hartley Channnel capacity theorem. Impliocation of the theorem and theoretical limits.
    3. Electrical rrepresentation of binary data (line coders), Unipolar NRZ, bipolar NRZ, unipolar RZ, bipolar RZ, Manchester (split phase), differential (binary RZ-alternate mark inversion) codes, properties, comparisons.
    4. Baseband data communication systems, Inter-symbol interference (ISI), pulse shaping (Nyquist, Raised-cosine) and bandwidth considerations.
    5. Correlative coding techniques, duobinary and modified duobinary encoders
    6. M-ary signaling, comparison with binary signaling.
    7. The eye diagram.

9. Bandpass (modulated) data communication systems (4 hours)
    1. Binary digital modulatons, ASK, FSK, PSK, DPSK, QPSK, GMPSK, implementations, properties and comparisons.
    2. M-ary data communication systems, quadrature amplitude modulation systems, four phase PSK systems.
    3. Demodulation of binary digital modulated signals (cohorent and non-cohorent).
    4. Modems and its applications.

10. Random signals and noise in communication systems (6 hours)
    1. Random variables and processes, random signals, statistical and time averaged moments, interpretation of time averaged moments of a random process stationary process, ergodic process, psdf and AC function of a ergodic random process.
    2. White noise, thermal noise, band-limited white noise, the psdf and AC function of white noise.
    3. Passage of wide-sense stationary random signals through a LTI.
    4. Ideal low-pass and RC filtering of white noise, noise equivalent bandwidth of a filter.
    5. Optimum detection of a pulse in additive white noise, the matched filter. Realization of matched filters (time co-relaters). The matched filter for a rectangular pulse, ideal LPF and RC filters as matched filters.
    6. Performance limitation of baseband data communications due to noise, error probabilities in binary and M-ary baseband data communication.

11. Noise performance of band-pass (modulated) communication system (6 hours)
    1. Effect of noise in envelop and synchronous demodulation of DSB-FC AM, expression for gain parameter (ratio of output SNR to input SNR), threshold effect in non-linear demodulation of AM.
    2. Gain parameter for demodulations of DSB-SC and SSB using synchronous demodulators.
    3. Effect of noise (gain parameter) for non-cohorent (limiter discriminator-envelop detector) demodulation of FM, threshold effect in FM. Use of pre-emphasis and de-emphais circuits in FM.
    4. Comparison of AM (SDB-FC, DSB-SC, SSB) and FM (Narrow and wide-bands) in terms of power efficiency, channel bandwidth and complexity.
    5. Noise performance of modulated digital systems. Error probabilites for ASK, FSK, PSK, DPSK with cohorent and non-cohorent demodulation.
    6. Comparison of modulated digital systems in terms of bandwidth efficiency, power efficiency and complexity.

12. Multiplexing (2 hours)
    1. Principle of frequency division multiplexing (FDM), FDM in telephnony, hierarchy.
    2. Frequency Division Multiple Access (FDMA) systems - SCPC, DAMA, SPADE etc.
    3. Filter and oscillator requirements in FDM.
    4. Time Division Multiplexing with PCM, data rate and bandwidth of a PCM signal.
    5. The T1 and E1 TDM PCM telephone hierarchy.

13. Error control coding techniques (4 hours)
    1. Basic principles of error control coding, types, basic definitions (hamming weight, hamming distance, minimum weight), hamming distance of error control capabilities.
    2. Linear block codes (systematic and non-systematic), generation, capabilities, syndrome calculation.
    3. Binary cyclic codes (symmetric and non-symmetric), generation, capabilities, syndrome calculation.
    4. Convolutional codes, implementation, code tree, trellis and decoding algorithms.

## Experiments (Suggested)

1. Demonstration of power spectrum of various signals using LF spectrum analyzer.
2. Generation of DSB-SC, DSB-FC and SSB signals.
3. Demodulation of AM signals (synchronous and non-synchronous methods).
4. Generation of FM signals.
5. Demodulation of FM signal (limiter-discriminator).
6. Operatin of PLL, PLL as demodulator of AM and FM signal.
7. Study of line codes.
8. Study of PCM.
9. Study of DPCM.
10. Study of DM.
11. Study of ASK, FSK and PSK.
12. Study of eye diagram.

## Reference books
    1. S Haykin, Digital communication systems, latest editions
    2. Leon Couch, Digital and analog communication systems, latest edition
    3. B.P. Lathi, Analog and Digital communication systems, latest edition
    4. J. Proakis, Digital communication systems, latest edition
    5. D. Sharma, Course manuals "communication Systems I" and "Communication Systems II"

## Evaluation Scheme

Mark distribution for all the chapters in the syllabus is shown in the table below:

| S.N.  | Unit          | Hours          | Marks Distrubution (%) |
| ----- | ------------- | -------------- | ---------------------- |
| 1     | 1,2 and 4     | 2 + 4 + 8 = 14 | 20%                    |
| 2     | 3 and 5       | 2 + 6 = 8      | 10%                    |
| 3     | 6 and 7       | 4 + 6 = 10     | 10%                    |
| 4     | 8, 9 and 10   | 6 + 4 + 6 = 16 | 20%                    |
| 5     | 11, 12 and 13 | 6 + 2 + 6 = 14 | 20%                    |
| Total |               | 62 hours       | 80%                    |

* There may be minor variations in the marks distributions.

## Attributions to the Contributors:

[Krischal Khanal](https://github.com/krischal111)