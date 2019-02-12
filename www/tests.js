function generate_seed(count) {
    let seed = new Uint8Array(count);
    for (let i = 0; i < count; i++) {
        seed[i] = Math.floor(Math.random() * 256)
    }
    return seed
} 

String.prototype.getBytes = function () {
    var bytes = [];
    for (var i = 0; i < this.length; ++i) {
        bytes.push(this.charCodeAt(i));
    }
    return bytes;
};

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
        let SCHNORRKEL = window.schnorrkel
        let seed = generate_seed(32)
        let pair = SCHNORRKEL.secret_from_seed(seed)
        chai.expect(pair.length).to.equal(64)
    })

    it('should sign a message', function () {
        let SCHNORRKEL = window.schnorrkel
        let message = "This is a message".getBytes()
        let seed = generate_seed(32)
        let keypair = SCHNORRKEL.keypair_from_seed(seed)
        let private = keypair.slice(0, 64)
        let public = keypair.slice(64, 96)
        let signature = SCHNORRKEL.sign(public, private, message)
        chai.expect(signature.length).to.equal(64)
    })

    it('should verify a signed message', function () {
        let SCHNORRKEL = window.schnorrkel
        let message = "This is a message".getBytes()
        let seed = generate_seed(32)
        let keypair = SCHNORRKEL.keypair_from_seed(seed)
        let private = keypair.slice(0, 64)
        let public = keypair.slice(64, 96)
        let signature = SCHNORRKEL.sign(public, private, message)
        chai.expect(SCHNORRKEL.verify(signature, message, public)).to.equal(true)
    })

    it('should reject a wrong signature', function() {
        let SCHNORRKEL = window.schnorrkel
        let message = "This is a message".getBytes()
        let seed = generate_seed(32)
        let keypair = SCHNORRKEL.keypair_from_seed(seed)
        let private = keypair.slice(0, 64)
        let public = keypair.slice(64, 96)
        let signature = SCHNORRKEL.sign(public, private, message)
        // mutate the signature
        signature[0] = (signature[0] + 1) % 255
        chai.expect(SCHNORRKEL.verify(signature, message, public)).to.equal(false)
    })
});