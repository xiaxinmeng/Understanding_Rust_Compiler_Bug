perl
use utf8;
{
  package Слово;
  sub new {
    my $self = bless {}, shift;
    $self->{стойност} = shift;
    $self
  }
};
my $здрасти = Слово->new("здравей, свят");
say ucfirst($здрасти->{стойност}); #Здравей, свят
