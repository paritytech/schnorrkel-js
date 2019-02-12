function generate_seed(count) {
    let seed = new Uint8Array(count);
    for (let i = 0; i < count; i++) {
        seed[i] = Math.floor(Math.random() * 256)
    }
    return seed
} 

describe('Basic Signing and Verification', function () {
    before(function (done) {
        setTimeout(function() {
            done()
        }, 500)
    });

    it('Should create a keypair', function () {
        let SCHNORRKEL = window.schnorrkel
        let seed = generate_seed(32)
        let pair = SCHNORRKEL.keypair_from_seed(seed)
        chai.expect(pair.length).to.equal(64 + 32)
    });

    it('should create a standalone secret key', function () {

    })

    it('should sign a message', function () {

    })

    it('should verify a signed message', function () {

    })

    it('should reject a wrong signature', function() {

    })
});