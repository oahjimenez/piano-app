
interface PianoSheet {

    _id : BsonId,
    chord_type : string,
    exemple : string,
    formula : string,
    sound_quality : string

}

interface BsonId {

    $oid : string

}
