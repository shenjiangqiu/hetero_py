## the data set and the metapaths
- DBLP Author, Paper, Term, Venue, [APA,APTPA,APVPA]
- IMDB, Movie, Director, Actor [MDM,MAM,DMD,DMAMD,AMA,AMDMA]
- LASTFM, User,Artist,Tag, [UAU,UATAU,AUA,ATA]
- OGB_MAG, Author,Paper,Instituion,Field,[APA,APFPA]
- OAG, Author,Paper,Institution,Field,Venue,[APA,APFPA]


aminer
HeteroData(
  author={
    y=[246678],
    y_index=[246678],
    num_nodes=1693531
  },
  venue={
    y=[134],
    y_index=[134],
    num_nodes=3883
  },
  paper={ num_nodes=3194405 },
  (paper, written_by, author)={ edge_index=[2, 9323605] },
  (author, writes, paper)={ edge_index=[2, 9323605] },
  (paper, published_in, venue)={ edge_index=[2, 3194405] },
  (venue, publishes, paper)={ edge_index=[2, 3194405] }
)
ogb-mag
HeteroData(
  paper={
    x=[736389, 128],
    year=[736389],
    y=[736389],
    train_mask=[736389],
    val_mask=[736389],
    test_mask=[736389]
  },
  author={ num_nodes=1134649 },
  institution={ num_nodes=8740 },
  field_of_study={ num_nodes=59965 },
  (author, affiliated_with, institution)={ edge_index=[2, 1043998] },
  (author, writes, paper)={ edge_index=[2, 7145660] },
  (paper, cites, paper)={ edge_index=[2, 10792672] },
  (paper, has_topic, field_of_study)={ edge_index=[2, 7505078] },
  (institution, rev_affiliated_with, author)={ edge_index=[2, 1043998] },
  (paper, rev_writes, author)={ edge_index=[2, 7145660] },
  (field_of_study, rev_has_topic, paper)={ edge_index=[2, 7505078] }
)
Processing...
Done!
dblp
HeteroData(
  author={
    x=[4057, 334],
    y=[4057],
    train_mask=[4057],
    val_mask=[4057],
    test_mask=[4057]
  },
  paper={ x=[14328, 4231] },
  term={ x=[7723, 50] },
  conference={ num_nodes=20 },
  (author, to, paper)={ edge_index=[2, 19645] },
  (paper, to, author)={ edge_index=[2, 19645] },
  (paper, to, term)={ edge_index=[2, 85810] },
  (paper, to, conference)={ edge_index=[2, 14328] },
  (term, to, paper)={ edge_index=[2, 85810] },
  (conference, to, paper)={ edge_index=[2, 14328] }
)
Processing...
Done!
imdb
HeteroData(
  movie={
    x=[4278, 3066],
    y=[4278],
    train_mask=[4278],
    val_mask=[4278],
    test_mask=[4278]
  },
  director={ x=[2081, 3066] },
  actor={ x=[5257, 3066] },
  (movie, to, director)={ edge_index=[2, 4278] },
  (movie, to, actor)={ edge_index=[2, 12828] },
  (director, to, movie)={ edge_index=[2, 4278] },
  (actor, to, movie)={ edge_index=[2, 12828] }
)
movie-lens
None
last-fm
HeteroData(
  user={ num_nodes=1892 },
  artist={ num_nodes=17632 },
  tag={ num_nodes=1088 },
  (user, to, artist)={
    train_neg_edge_index=[2, 33294760],
    val_pos_edge_index=[2, 9283],
    val_neg_edge_index=[2, 9283],
    test_pos_edge_index=[2, 18567],
    test_neg_edge_index=[2, 18567],
    edge_index=[2, 64984]
  },
  (user, to, user)={ edge_index=[2, 25434] },
  (artist, to, user)={ edge_index=[2, 64984] },
  (artist, to, tag)={ edge_index=[2, 23253] },
  (tag, to, artist)={ edge_index=[2, 23253] }
)
hgb
HeteroData(
  paper={
    x=[3025, 1902],
    y=[3025],
    train_mask=[3025],
    test_mask=[3025]
  },
  author={ x=[5959, 1902] },
  subject={ x=[56, 1902] },
  term={ num_nodes=1902 },
  (paper, cite, paper)={ edge_index=[2, 5343] },
  (paper, ref, paper)={ edge_index=[2, 5343] },
  (paper, to, author)={ edge_index=[2, 9949] },
  (author, to, paper)={ edge_index=[2, 9949] },
  (paper, to, subject)={ edge_index=[2, 3025] },
  (subject, to, paper)={ edge_index=[2, 3025] },
  (paper, to, term)={ edge_index=[2, 255619] },
  (term, to, paper)={ edge_index=[2, 255619] }
)
taobao
HeteroData(
  user={ num_nodes=987991 },
  item={ num_nodes=4161138 },
  category={ num_nodes=9437 },
  (user, to, item)={
    edge_index=[2, 100095182],
    time=[100095182],
    behavior=[100095182]
  },
  (item, to, category)={ edge_index=[2, 4162555] },
  (item, rev_to, user)={
    edge_index=[2, 100095182],
    time=[100095182],
    behavior=[100095182]
  },
  (category, rev_to, item)={ edge_index=[2, 4162555] }
)